//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1184/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1184<F: Float>(t33017: F, t6986: F, t1799: F, t6662: F, t9679: F, t6668: F, t6676: F, t5054: F, t6680: F, t32921: F, t32996: F, t33008: F, t34122: F, t34182: F, t34192: F, t9649: F, t9652: F, t9672: F, t9922: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t34200 = t33017 * t6986;
    let t34201 = t1799 * t34200;
    let t34203 = t9679 * t6662;
    let t34204 = t1799 * t34203;
    let t34206 = t9679 * t6668;
    let t34207 = t1799 * t34206;
    let t34209 = t9679 * t6676;
    let t34210 = t5054 * t34209;
    let t34212 = t33017 * t6680;
    let t34213 = t1799 * t34212;
    let t34215 = 0.34722222222222222223e-2 * t32996 + 0.40208333333333333335e-2 * t32921 * t9922 + 0.10416666666666666667e-1 * t34122 * t9652 + 0.40208333333333333335e-2 * t34192 * t9652 - 0.40208333333333333335e-2 * t9649 * t34182 + 0.16581944444444444444e-2 * t33008 + 0.10416666666666666667e-1 * t34122 * t9672 - 0.16581944444444444444e-2 * t34201 + 0.11054629629629629629e-2 * t34204 - 0.33163888888888888888e-2 * t34207 + 0.27636574074074074073e-2 * t34210 - 0.16581944444444444444e-2 * t34213;
    (t34200, t34201, t34203, t34204, t34206, t34207, t34209, t34210, t34212, t34213, t34215)
}
