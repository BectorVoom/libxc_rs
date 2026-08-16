//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2263/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2263(t3400: f64, t6063: f64, t1098: f64, t18245: f64, t3312: f64, t5983: f64, t18496: f64, t699: f64, t18517: f64, t18514: f64, t18520: f64, t2403: f64, t6011: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t63602 = t6063 * t3400;
    let t63750 = t18245 * t1098;
    let t63755 = t5983 * t3312;
    let t63841 = t699 * t18496;
    let t63843 = t699 * t18517;
    let t63845 = t699 * t18514;
    let t63886 = t699 * t18520;
    let t63888 = t2403 * t6011;
    (t63602, t63750, t63755, t63841, t63843, t63845, t63886, t63888)
}
