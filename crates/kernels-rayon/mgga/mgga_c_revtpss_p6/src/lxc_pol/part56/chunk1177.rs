//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1177/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1177(t131394: f64, t3089: f64, t33411: f64, t1121: f64, t494: f64, t1203: f64, t1469: f64, t34989: f64, t370: f64, t8923: f64, t124611: f64, t124613: f64, t124621: f64, t124635: f64, t124711: f64, t124755: f64, t1248: f64, t124959: f64, t1250: f64, t1791: f64, t1828: f64, t31993: f64, t32015: f64, t33417: f64, t33425: f64, t33426: f64, t33428: f64, t33495: f64, t33502: f64, t33524: f64, t3626: f64, t371: f64, t3719: f64, t372: f64, t482: f64, t5056: f64, t5215: f64, t5297: f64, t5320: f64, t5396: f64, t5422: f64, t5428: f64) -> (f64, f64) {
    let t131435 = t33411 * t131394 * t3089;
    let t131438 = t494 * t1121;
    let t131439 = t1469 * t1203;
    let t131467 = t8923 * t34989 * t370;
    let t131474 = 0.5578099381357651623e-3_f64 * t33502 * t5320 - 0.28234466758480466999e-3_f64 * t124611 * t124613 * t1828 * t1248 * t1250 - 0.15058382271189582399e-2_f64 * t131435 * t33417 - 0.37645955677973955998e-3_f64 * t124711 * t3626 * t131438 * t131439 + 0.37645955677973955998e-3_f64 * t124755 * t3626 * t131438 * t5297 + 0.5578099381357651623e-3_f64 * t124959 * t1791 - 0.5578099381357651623e-3_f64 * t33495 * t371 * t372 * t482 * t5215 + 0.12395776403017003607e-3_f64 * t33524 * t31993 * t5396 - 0.11156198762715303246e-2_f64 * t124621 * t31993 * t3719 * t5422 - 0.11156198762715303246e-2_f64 * t124635 * t31993 * t3719 * t5428 + 0.10038921514126388266e-2_f64 * t131467 * t33428 - 0.18822977838986977999e-3_f64 * t33425 * t32015 * t33426 * t5056;
    (t131439, t131474)
}
