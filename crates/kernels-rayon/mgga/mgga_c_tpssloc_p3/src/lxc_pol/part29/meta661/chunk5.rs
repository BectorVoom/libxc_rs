//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2200/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2200(t26215: f64, t81228: f64, t81326: f64, t16436: f64, t1985: f64, t6889: f64, t6906: f64, t2015: f64, t40590: f64, t6897: f64, t6907: f64, t90544: f64) -> (f64, f64, f64, f64) {
    let t90686 = t81228 * t81326 * t26215;
    let t90687 = 0.16449340668482264365e-1_f64 * t90686;
    let t90690 = t1985 * t6889 * t6906 * t16436;
    let t90696 = t40590 * t2015;
    let t90701 = t6897 * t90544 * t6907;
    (t90687, t90690, t90696, t90701)
}
