//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 413/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk413<F: Float>(t169: F, t171: F, t829: F, t167: F, t740: F, t113: F, t176: F, t833: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t2628 = t171 * t171;
    let t2629 = F::new(1.0) / t2628;
    let t2630 = t829 * t829;
    let t2633 = t167 * t740;
    let t2635 = -F::new(2.0) * t113 + F::new(2.0) * t2633;
    let t2639 = piecewise3::<f64>(t170, F::new(0.0), F::new(4.0) / F::new(9.0) * t2629 * t2630 + F::new(4.0) / F::new(3.0) * t171 * t2635);
    let t2640 = t176 * t176;
    let t2641 = F::new(1.0) / t2640;
    let t2642 = t833 * t833;
    (t2628, t2629, t2630, t2633, t2635, t2639, t2640, t2641, t2642)
}
