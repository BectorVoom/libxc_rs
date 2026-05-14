//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 908/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk908<F: Float>(t2484: F, t4663: F, t1646: F, t6787: F, t16034: F, t10671: F, t2372: F, t4624: F, t15991: F, t6771: F, t1648: F, t4652: F, t6777: F, t11371: F, t15989: F, t15993: F, t15996: F, t16001: F, t16011: F, t16015: F, t16032: F) -> (F, F, F, F, F, F, F) {
    let t16037 = t4663 * t2484;
    let t16040 = t1646 * t6787;
    let t16045 = t1646 * t16034;
    let t16047 = t10671 * t2372;
    let t16048 = t16047 * t4624;
    let t16061 = 0.18344444444444444444e-2 * t15991;
    let t16067 = t4663 * t6771;
    let t16068 = t16067 * t1648;
    let t16070 = t6777 * t4652;
    let t16072 = -0.18344444444444444444e-2 * t15989 - 0.55033333333333333333e-2 * t15993 + t16061 - 0.27516666666666666667e-2 * t16015 - 0.45861111111111111112e-2 * t16001 + 0.11006666666666666667e-1 * t16011 - t11371 - 0.30268333333333333334e-1 * t15996 + 0.8255e-2 * t16032 - 0.1982e-1 * t16068 - 0.991e-2 * t16070;
    (t16037, t16040, t16045, t16048, t16068, t16070, t16072)
}
