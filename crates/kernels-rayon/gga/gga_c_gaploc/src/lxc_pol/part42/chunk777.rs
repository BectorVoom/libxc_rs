//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 777/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk777(t12704: f64, t2464: f64, t2684: f64, t1645: f64, t7696: f64, t22980: f64, t2615: f64, t9438: f64, t22984: f64, t7584: f64, t12692: f64, t2013: f64) -> (f64, f64, f64, f64, f64) {
    let t41071 = t2684 * t2464 * t12704;
    let t41105 = t1645 * t7696;
    let t41231 = t2615 * t9438 * t22980;
    let t41244 = t7584 * t9438 * t22984;
    let t41295 = t2013 * t12692;
    (t41071, t41105, t41231, t41244, t41295)
}
