//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 979/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk979<F: Float>(t1676: F, t1685: F, t22745: F, t45: F, t8584: F, t10696: F, t8590: F, t10699: F, t1683: F, t4787: F, t8607: F, t6880: F, t18565: F, t6879: F, t10569: F, t10570: F, t15989: F, t15991: F, t15996: F, t16528: F, t22564: F, t22567: F, t22570: F, t22573: F, t22575: F, t22578: F, t22581: F, t22583: F, t22586: F, t22589: F, t22594: F) -> (F, F, F, F, F, F) {
    let t22747 = t1676 * t22745 * t1685;
    let t22750 = t45 * t8584;
    let t22755 = t10696 * t8590;
    let t22756 = t10699 * t1683;
    let t22757 = t22755 * t22756;
    let t22760 = t4787 * t8607;
    let t22761 = t22760 * t6880;
    let t22764 = t6879 * t18565;
    let t22784 = -t10569 - 0.79148148148148148147e-2 * t10570 - 0.15829629629629629629e-1 * t15989 + 0.79148148148148148147e-2 * t15991 - t16528 - 0.23744444444444444444e-1 * t15996 + 0.39574074074074074073e-2 * t22564 - 0.19787037037037037037e-1 * t22567 + 0.71233333333333333332e-1 * t22570 + 0.47488888888888888888e-1 * t22573 - 0.11872222222222222222e-1 * t22575 - 0.10685e0 * t22578 - 0.14246666666666666666e0 * t22581 + 0.5936111111111111111e-2 * t22583 - 0.11872222222222222222e-1 * t22586 + 0.35616666666666666666e-1 * t22589 - 0.17808333333333333333e-1 * t22594;
    (t22747, t22750, t22757, t22761, t22764, t22784)
}
