//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1137/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1137(t3131: f64, t5658: f64, t1084: f64, t29568: f64, t11781: f64, t3368: f64, t11892: f64, t11473: f64, t3322: f64, t3363: f64, t3330: f64, t33560: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34058 = t3131 * t5658;
    let t34060 = t1084 * t34058 * t29568;
    let t34062 = t11781 * t3368;
    let t34066 = t11892 * t3368;
    let t34069 = t3363 * t11473 * t3322;
    let t34071 = t33560 * t3330;
    (t34058, t34060, t34062, t34066, t34069, t34071)
}
