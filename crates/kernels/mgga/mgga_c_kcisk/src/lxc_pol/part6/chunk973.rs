//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 973/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk973<F: Float>(t222: F, t227: F, t15772: F, t224: F, t30153: F, t30158: F, t5562: F, t7710: F, t15783: F, t229: F, t28312: F, t28368: F, t5570: F, t7718: F, zeta_threshold: F) -> (F, F) {
    let t223 = t222 <= zeta_threshold;
    let t228 = t227 <= zeta_threshold;
    let t30162 = piecewise3::<f64>(t223, F::new(0.0), -F::new(8.0) / F::new(27.0) * t15772 * t30153 + F::new(4.0) / F::new(3.0) * t5562 * t7710 + F::new(4.0) / F::new(3.0) * t224 * t30158);
    let t30170 = piecewise3::<f64>(t228, F::new(0.0), -F::new(8.0) / F::new(27.0) * t15783 * t28368 + F::new(4.0) / F::new(3.0) * t5570 * t7718 + F::new(4.0) / F::new(3.0) * t229 * t28312);
    (t30162, t30170)
}
