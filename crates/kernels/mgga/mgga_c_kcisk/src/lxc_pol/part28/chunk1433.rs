//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1433/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1433<F: Float>(t35475: F, t4419: F, t9725: F, t10000: F, t34465: F, t113058: F, t118246: F, t118248: F, t118250: F, t121693: F, t121702: F, t121705: F, t121708: F, t121715: F, t24934: F, t33219: F, t34444: F, t34469: F, t34499: F, t35416: F, t35446: F, t9733: F, t9740: F) -> (F, F) {
    let t122980 = t4419 * t35475;
    let t122981 = t9725 * t122980;
    let t122986 = t10000 * t34465;
    let t122995 = -0.34722222222222222222e-2 * t9740 * t33219 * t34499 * t24934 - 0.25794135802469135802e-3 * t121693 - t118246 - t118248 + 0.23148148148148148148e-2 * t118250 + 0.6701388888888888889e-3 * t122981 + 0.77382407407407407407e-3 * t121702 - 0.23214722222222222222e-2 * t121705 - 0.11607361111111111111e-2 * t121708 + 0.34722222222222222223e-2 * t122986 + t113058 - 0.10416666666666666667e-1 * t9733 * t35416 + 0.40208333333333333334e-2 * t34444 * t34469 - 0.10416666666666666667e-1 * t9733 * t35446 - 0.46429444444444444444e-2 * t121715;
    (t122980, t122995)
}
