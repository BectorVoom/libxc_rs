//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1034/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1034(t31295: f64, t6876: f64, t2363: f64, t652: f64, t8595: f64, t24432: f64, t24995: f64, t90065: f64, t31776: f64, t91669: f64, t2320: f64, t31300: f64, t83886: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115738 = 2.0_f64 * t6876 * t31295;
    let t115743 = 2.0_f64 * t652 * t8595 * t2363;
    let t115748 = 6.0_f64 * t24995 * t24432 * t90065;
    let t115750 = 4.0_f64 * t91669 * t31776;
    let t115752 = 2.0_f64 * t2320 * t8595;
    let t115754 = 6.0_f64 * t83886 * t31300;
    (t115738, t115743, t115748, t115750, t115752, t115754)
}
