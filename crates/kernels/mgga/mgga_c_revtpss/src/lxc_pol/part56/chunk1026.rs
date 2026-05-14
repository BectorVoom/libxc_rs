//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1026/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1026<F: Float>(t131394: F, t3089: F, t33411: F, t1121: F, t494: F, t1203: F, t1469: F, t34989: F, t370: F, t8923: F, t124611: F, t124613: F, t124621: F, t124635: F, t124711: F, t124755: F, t1248: F, t124959: F, t1250: F, t1791: F, t1828: F, t31993: F, t32015: F, t33417: F, t33425: F, t33426: F, t33428: F, t33495: F, t33502: F, t33524: F, t3626: F, t371: F, t3719: F, t372: F, t482: F, t5056: F, t5215: F, t5297: F, t5320: F, t5396: F, t5422: F, t5428: F) -> (F, F) {
    let t131435 = t33411 * t131394 * t3089;
    let t131438 = t494 * t1121;
    let t131439 = t1469 * t1203;
    let t131467 = t8923 * t34989 * t370;
    let t131474 = 0.5578099381357651623e-3 * t33502 * t5320 - 0.28234466758480466999e-3 * t124611 * t124613 * t1828 * t1248 * t1250 - 0.15058382271189582399e-2 * t131435 * t33417 - 0.37645955677973955998e-3 * t124711 * t3626 * t131438 * t131439 + 0.37645955677973955998e-3 * t124755 * t3626 * t131438 * t5297 + 0.5578099381357651623e-3 * t124959 * t1791 - 0.5578099381357651623e-3 * t33495 * t371 * t372 * t482 * t5215 + 0.12395776403017003607e-3 * t33524 * t31993 * t5396 - 0.11156198762715303246e-2 * t124621 * t31993 * t3719 * t5422 - 0.11156198762715303246e-2 * t124635 * t31993 * t3719 * t5428 + 0.10038921514126388266e-2 * t131467 * t33428 - 0.18822977838986977999e-3 * t33425 * t32015 * t33426 * t5056;
    (t131439, t131474)
}
