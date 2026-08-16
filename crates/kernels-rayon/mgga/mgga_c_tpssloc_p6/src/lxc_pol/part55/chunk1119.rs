//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1119/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1119(t112: f64, t34228: f64, t225: f64, t497: f64, t8054: f64, t462: f64, t1716: f64, t8867: f64, t27751: f64, t8871: f64, t32543: f64, t8014: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34229 = t34228 * t112;
    let t34237 = t8054 * t225 * t497;
    let t34238 = t462 * t34237;
    let t34241 = t1716 * t8867;
    let t34244 = t27751 * t8871;
    let t34247 = t32543 * t8014;
    (t34229, t34237, t34238, t34241, t34244, t34247)
}
