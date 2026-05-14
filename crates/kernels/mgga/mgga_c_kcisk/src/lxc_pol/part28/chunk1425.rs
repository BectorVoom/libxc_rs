//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1425/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1425<F: Float>(t35427: F, t9736: F, t2028: F, t33197: F, t7261: F, t9226: F, t113124: F, t2364: F, t34398: F, t118103: F, t34404: F, t1060: F, t2647: F, t118185: F, t6763: F, t113181: F, t118120: F, t118275: F, t121405: F, t2807: F, t34412: F, t34416: F, t34424: F, t34429: F, t34496: F, t34501: F, t34552: F, t34563: F, t34579: F, t9740: F, t9990: F) -> (F, F, F, F, F, F) {
    let t122741 = t35427 * t9736;
    let t122745 = t7261 * t33197 * t9226 * t2028;
    let t122755 = t113124 * t2364 * t34398;
    let t122759 = t118103 * t2364 * t34404;
    let t122762 = t2647 * t1060;
    let t122764 = t118185 * t6763 * t122762;
    let t122771 = 0.34722222222222222222e-2 * t34416 * t34552 + 0.34722222222222222222e-2 * t34416 * t34496 + 0.69444444444444444444e-2 * t34416 * t34501 + 0.13402777777777777778e-2 * t118275 * t34496 - 0.46296296296296296296e-2 * t34416 * t34563 - 0.34722222222222222223e-2 * t122741 - 0.52083333333333333333e-2 * t9740 * t122745 + 0.55555555555555555556e-1 * t34412 * t34424 + 0.27777777777777777778e-1 * t34412 * t34429 + 0.10722222222222222222e-1 * t118120 * t34429 - 0.34722222222222222222e-2 * t113181 * t122755 - 0.69444444444444444444e-2 * t113181 * t122759 - 0.69444444444444444444e-2 * t113181 * t122764 + 0.27777777777777777778e-1 * t9990 * t34579 * t2807 - 0.15476481481481481481e-2 * t121405;
    (t122745, t122755, t122759, t122762, t122764, t122771)
}
