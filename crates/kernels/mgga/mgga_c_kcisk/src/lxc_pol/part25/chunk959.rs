//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 959/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk959<F: Float>(t2442: F, t5068: F, t5184: F, t15891: F, t11200: F, t2469: F, t4824: F, t1692: F, t6884: F, t10527: F, t10532: F, t11209: F, t11211: F, t16702: F, t16705: F, t16714: F, t16719: F, t1693: F, t16983: F, t16986: F, t16989: F, t16992: F, t16997: F, t1792: F, t4823: F, t4830: F, t7275: F) -> (F, F, F, F, F, F) {
    let t17000 = t2442 * t5068;
    let t17001 = t5184 * t17000;
    let t17002 = t15891 * t17001;
    let t17004 = t2469 * t11200;
    let t17005 = t17004 * t4824;
    let t17010 = t6884 * t1692;
    let t17013 = -0.73697530864197530861e-3 * t16702 - 0.33163888888888888888e-2 * t16705 + 0.22109259259259259258e-2 * t10527 - 0.16581944444444444444e-2 * t10532 - 0.386e0 * t4830 * t7275 + 0.55273148148148148147e-3 * t16714 - 0.1492375e-1 * t16719 - 0.24872916666666666666e-2 * t16983 + 0.16581944444444444444e-2 * t16986 - 0.24872916666666666666e-2 * t16989 + 0.386e0 * t1693 * t16992 + 0.148996e0 * t4823 * t16992 + 0.193e0 * t1693 * t16997 + 0.11054629629629629629e-2 * t17002 - 0.386e0 * t1693 * t17005 + 0.11054629629629629629e-2 * t11209 - 0.73697530864197530861e-3 * t11211 - 0.386e0 * t17010 * t1792;
    (t17000, t17002, t17004, t17005, t17010, t17013)
}
