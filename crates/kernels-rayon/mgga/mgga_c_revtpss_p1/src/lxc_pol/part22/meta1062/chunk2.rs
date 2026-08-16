//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3794/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3794(t21342: f64, t460: f64, t1204: f64, t12633: f64, t12641: f64, t1274: f64, t1295: f64, t13182: f64, t1775: f64, t18037: f64, t18062: f64, t18109: f64, t1829: f64, t20704: f64, t20714: f64, t20741: f64, t20756: f64, t21344: f64, t3552: f64, t3556: f64, t3738: f64, t3739: f64, t5237: f64, t5417: f64, t5423: f64, t56396: f64, t56575: f64, t6697: f64, t6744: f64) -> f64 {
    let t72959 = t460 * t21342;
    let t72986 = 0.26341796731742046394e1_f64 * t18037 * t5423 - 0.13170898365871023197e1_f64 * t72959 * t1295 + 0.13170898365871023197e1_f64 * t1204 * t21344 - 0.26341796731742046394e1_f64 * t3556 * t20741 - 0.39512695097613069591e1_f64 * t1274 * t13182 * t6744 * t3738 + 0.52683593463484092788e1_f64 * t5417 * t18109 + 0.26341796731742046394e1_f64 * t12641 * t20704 - 0.26341796731742046394e1_f64 * t12633 * t20714 + 0.26341796731742046394e1_f64 * t18062 * t5237 + 0.65854491829355115987e0_f64 * t3552 * t6697 + 0.26341796731742046394e1_f64 * t20756 * t3739 - 0.13170898365871023197e1_f64 * t56396 * t1775 - 0.26341796731742046394e1_f64 * t56575 * t1829;
    t72986
}
