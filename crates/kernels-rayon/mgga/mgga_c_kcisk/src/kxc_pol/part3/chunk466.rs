//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 466/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk466(t1379: f64, t311: f64, t313: f64, t1187: f64, t827: f64, t1311: f64, t79: f64, t3575: f64, t26: f64, t1186: f64, t3579: f64, t3583: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3657 = t311 * t1379 * t313;
    let t3658 = 0.13692777777777777778e0_f64 * t3657;
    let t3659 = t827 * t1187;
    let t3661 = t79 * t1311;
    let t3662 = t3661 * t3575;
    let t3663 = t26 * t3662;
    let t3665 = t1186 * t3579;
    let t3666 = t26 * t3665;
    let t3668 = t1186 * t3583;
    (t3657, t3658, t3659, t3661, t3662, t3663, t3665, t3666, t3668)
}
