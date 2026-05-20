//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1468/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1468<F: Float>(t2935: F, t2942: F, t11452: F, t11453: F, t11456: F, t11461: F, t11466: F, t11502: F, t11509: F, t11510: F, t11557: F, t2945: F, t2968: F, t2970: F, t2982: F, t2987: F, t3007: F, t3015: F, t41225: F, t41238: F, t41464: F, t41505: F, t41658: F, t41662: F, t41667: F, t41668: F, t41686: F, t41701: F, t41717: F, t41732: F, t41740: F, t41742: F, t41746: F, t41751: F, t41756: F, t41759: F, t41763: F, t946: F, t954: F, t973: F, t974: F) -> F {
    let t41775 = t2935 * t2942;
    let t41778 = F::cast_from(0.91082604192152556044e5_f64) * t41658 * t41225 * t41238 + F::cast_from(0.82761620670837440481e4_f64) * t41662 * t11453 - F::cast_from(0.24828486201251232145e5_f64) * t41667 * t41668 * t11452 + F::new(1.0) * t946 * (t41686 + t41701 + t41717 + t41732) * t954 + F::cast_from(0.19964560303604640732e6_f64) * t41740 * t41668 * t41742 + F::cast_from(0.23392894490538584828e1_f64) * t41746 * t974 + F::cast_from(0.35089341735807877242e1_f64) * t11456 * t3007 + F::cast_from(0.10389515463408878255e3_f64) * t41751 * t3015 + F::cast_from(0.23392894490538584828e1_f64) * t2982 * t11502 + F::cast_from(0.4101607543286562663e4_f64) * t41756 * t11510 - F::cast_from(0.12304822629859687989e5_f64) * t41759 * t41225 * t11509 + F::cast_from(0.96491876992155210402e2_f64) * t2968 * t41763 * t2970 + F::cast_from(0.14035736694323150897e2_f64) * t11461 * t11557 - F::cast_from(0.14035736694323150897e2_f64) * t11466 * t41225 * t973 - F::cast_from(0.35089341735807877242e1_f64) * t2987 * t41464 * t973 - t41505 - F::new(12.0) * t41775 * t2945;
    t41778
}
