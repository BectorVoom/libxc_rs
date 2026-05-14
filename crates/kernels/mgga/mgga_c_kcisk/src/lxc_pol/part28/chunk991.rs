//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 991/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk991<F: Float>(t22984: F, t5184: F, t5182: F, t1636: F, t9089: F, t10365: F, t15858: F, t6680: F, t15892: F, t6702: F, t15891: F, t1755: F, t2527: F, t6707: F, t10474: F, t10502: F, t11245: F, t16640: F, t16643: F, t22371: F, t22955: F, t22960: F, t22963: F, t22966: F, t22968: F, t22973: F, t22977: F, t22980: F, t4830: F, t8846: F, t8852: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22985 = t5184 * t22984;
    let t22986 = t5182 * t22985;
    let t22988 = t9089 * t1636;
    let t22989 = t10365 * t22988;
    let t22990 = t5182 * t22989;
    let t22992 = t15858 * t6680;
    let t22993 = t5182 * t22992;
    let t22995 = t15892 * t6702;
    let t22996 = t10365 * t22995;
    let t22997 = t15891 * t22996;
    let t22999 = t1755 * t2527;
    let t23000 = t22999 * t6707;
    let t23001 = t5184 * t23000;
    let t23002 = t15891 * t23001;
    let t23008 = -0.33163888888888888888e-2 * t22371 - 0.24872916666666666666e-2 * t22955 + 0.66327777777777777776e-2 * t22960 + 0.17687407407407407407e-1 * t22963 - 0.14739506172839506172e-1 * t22966 + 0.22109259259259259258e-2 * t22968 - 0.44218518518518518516e-2 * t22973 - 0.36848765432098765431e-3 * t10474 - 0.33163888888888888888e-2 * t22977 - 0.88437037037037037033e-2 * t22980 + t10502 - 0.11054629629629629629e-2 * t16640 - 0.22109259259259259259e-2 * t16643 - 0.16581944444444444444e-2 * t22986 + 0.33163888888888888888e-2 * t22990 + 0.88437037037037037033e-2 * t22993 + 0.66327777777777777776e-2 * t22997 + 0.11054629629629629629e-2 * t23002 - 0.193e0 * t4830 * t8846 + 0.74498e-1 * t11245 * t8852;
    (t22986, t22988, t22990, t22993, t22995, t22997, t23000, t23002, t23008)
}
