//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 738/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk738(t2464: f64, t6977: f64, t2476: f64, t2482: f64, t589: f64, t587: f64, t1430: f64, t1441: f64, t1537: f64, t1599: f64, t2446: f64, t2449: f64, t2457: f64, t2493: f64, t4418: f64, t4428: f64, t4849: f64, t557: f64, t6937: f64, t6944: f64, t6950: f64, t6954: f64, t6957: f64, t6959: f64, t6961: f64, t6963: f64, t6965: f64, t6968: f64, t6972: f64, t6975: f64) -> (f64, f64, f64) {
    let t6978 = t2464 * t6977;
    let t6979 = t2476 * t6978;
    let t6981 = t589 * t2482;
    let t6982 = t587 * t6981;
    let t6984 = -0.47667319935800568892e0_f64 * t1599 * t2449 - 0.71500979903700853338e0_f64 * t1599 * t2446 - 0.35750489951850426669e0_f64 * t557 * t6937 + 0.51123901271894332905e0_f64 * t4418 * t2493 + 0.1022478025437886658e1_f64 * t4428 * t2457 + 0.1022478025437886658e1_f64 * t1441 * t6944 - 0.1022478025437886658e1_f64 * t4849 * t2493 - 0.1022478025437886658e1_f64 * t1537 * t6950 + 0.23833659967900284446e0_f64 * t6954 * t1430 - 0.59584149919750711116e-1_f64 * t6957 + 0.29792074959875355558e-1_f64 * t6959 + 0.14896037479937677779e-1_f64 * t6961 - 0.14300195980740170668e1_f64 * t6963 * t6965 - 0.29792074959875355558e-1_f64 * t6968 - 0.14896037479937677779e-1_f64 * t6972 + 0.19171462976960374838e0_f64 * t6975 - 0.42603251059911944086e-1_f64 * t6979 + 0.51123901271894332903e0_f64 * t6982;
    (t6979, t6982, t6984)
}
