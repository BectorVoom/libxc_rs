//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2271/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2271(t18321: f64, t3432: f64, t11529: f64, t1174: f64, t6130: f64, t15282: f64, t4889: f64, t18558: f64, t3431: f64, t11570: f64, t17686: f64, t15299: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t64976 = t18321 * t3432;
    let t64979 = t1174 * t11529 * t6130;
    let t64981 = t4889 * t15282;
    let t64988 = t1174 * t3431 * t18558;
    let t64994 = t11570 * t17686;
    let t65002 = t4889 * t15299;
    (t64976, t64979, t64981, t64988, t64994, t65002)
}
