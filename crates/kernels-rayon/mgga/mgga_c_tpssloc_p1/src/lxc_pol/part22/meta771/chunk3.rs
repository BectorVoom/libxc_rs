//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2628/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2628(t20217: f64, t3450: f64, t18469: f64, t3447: f64, t52059: f64, t4904: f64, t64763: f64, t18532: f64, t4889: f64, t1174: f64, t135: f64, t22040: f64) -> (f64, f64, f64, f64, f64) {
    let t73405 = t3450 * t20217;
    let t73417 = t3447 * t52059 * t18469;
    let t73420 = t3447 * t64763 * t4904;
    let t73424 = t4889 * t18532;
    let t73427 = t1174 * t135 * t22040;
    (t73405, t73417, t73420, t73424, t73427)
}
