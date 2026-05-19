//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 969/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk969<F: Float>(t227: F, t30125: F, t564: F, t2671: F, t8464: F, t742: F, t807: F, t2361: F, t28312: F, t565: F, t806: F, t2356: F, t8476: F, sigma2: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t228 = t227 <= zeta_threshold;
    let t30126 = t564 * t30125;
    let t30127 = F::new(3.0) / F::new(16.0) * t30126;
    let t30128 = t8464 * t2671;
    let t30129 = F::new(3.0) / F::new(8.0) * t30128;
    let t30130 = F::new(1.0) / t742;
    let t30131 = sigma2 * t30130;
    let t30132 = t30131 * t807;
    let t30133 = F::new(3.0) / F::new(8.0) * t30132;
    let t30134 = t8464 * t2361;
    let t30135 = F::new(3.0) / F::new(8.0) * t30134;
    let t30136 = piecewise3::<F>(t228, F::new(0.0), t28312);
    let t30137 = t565 * t30136;
    let t30138 = t30137 * t806;
    let t30139 = t564 * t30138;
    let t30140 = t30139 / F::new(16.0);
    let t30141 = t2356 * t8476;
    (t30127, t30129, t30133, t30135, t30140, t30141)
}
