//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1018/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1018<F: Float>(t2558: F, t5060: F, t5286: F, t1944: F, t5320: F, t7312: F, t11763: F, t7307: F, t1849: F, t2505: F, t1919: F, t3290: F, t16295: F, t673: F, t1648: F, t1824: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t17982 = t2558 * t5060;
    let t17983 = t17982 * sigma2;
    let t17984 = t17983 * t5286;
    let t17986 = t1944 * t5320;
    let t17987 = t17986 * t7312;
    let t17989 = t11763 * t7307;
    let t17991 = t2505 * t1849;
    let t17993 = t1919 * t17991 * t3290;
    let t17996 = t673 * t16295;
    let t18000 = t1648 * t1824;
    (t17982, t17984, t17987, t17989, t17993, t17996, t18000)
}
