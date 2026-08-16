//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2293/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2293(t3545: f64, t6109: f64, t13969: f64, t19071: f64, t3515: f64, t11728: f64, t18306: f64, t11738: f64, t19076: f64, t18940: f64, t486: f64, t15753: f64, t4889: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t66500 = t6109 * t3545;
    let t66512 = t3515 * t13969 * t19071;
    let t66515 = t11728 * t13969 * t18306;
    let t66518 = t11738 * t13969 * t19076;
    let t66533 = t486 * t18940;
    let t66545 = t4889 * t15753;
    (t66500, t66512, t66515, t66518, t66533, t66545)
}
