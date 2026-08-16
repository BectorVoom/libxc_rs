//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1468/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1468(t2935: f64, t2942: f64, t11452: f64, t11453: f64, t11456: f64, t11461: f64, t11466: f64, t11502: f64, t11509: f64, t11510: f64, t11557: f64, t2945: f64, t2968: f64, t2970: f64, t2982: f64, t2987: f64, t3007: f64, t3015: f64, t41225: f64, t41238: f64, t41464: f64, t41505: f64, t41658: f64, t41662: f64, t41667: f64, t41668: f64, t41686: f64, t41701: f64, t41717: f64, t41732: f64, t41740: f64, t41742: f64, t41746: f64, t41751: f64, t41756: f64, t41759: f64, t41763: f64, t946: f64, t954: f64, t973: f64, t974: f64) -> f64 {
    let t41775 = t2935 * t2942;
    let t41778 = 0.91082604192152556044e5_f64 * t41658 * t41225 * t41238 + 0.82761620670837440481e4_f64 * t41662 * t11453 - 0.24828486201251232145e5_f64 * t41667 * t41668 * t11452 + 1.0_f64 * t946 * (t41686 + t41701 + t41717 + t41732) * t954 + 0.19964560303604640732e6_f64 * t41740 * t41668 * t41742 + 0.23392894490538584828e1_f64 * t41746 * t974 + 0.35089341735807877242e1_f64 * t11456 * t3007 + 0.10389515463408878255e3_f64 * t41751 * t3015 + 0.23392894490538584828e1_f64 * t2982 * t11502 + 0.4101607543286562663e4_f64 * t41756 * t11510 - 0.12304822629859687989e5_f64 * t41759 * t41225 * t11509 + 0.96491876992155210402e2_f64 * t2968 * t41763 * t2970 + 0.14035736694323150897e2_f64 * t11461 * t11557 - 0.14035736694323150897e2_f64 * t11466 * t41225 * t973 - 0.35089341735807877242e1_f64 * t2987 * t41464 * t973 - t41505 - 12.0_f64 * t41775 * t2945;
    t41778
}
