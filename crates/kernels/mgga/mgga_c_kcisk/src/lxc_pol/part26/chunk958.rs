//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 958/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk958<F: Float>(t2075: F, t6211: F, t3937: F, t1322: F, t2168: F, t13472: F, t1056: F, t5675: F, t5670: F, t13504: F, t5601: F, t20067: F, t7740: F, t25441: F, t6183: F, t20053: F, t25450: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26029 = t2075 * t6211;
    let t26030 = t3937 * t26029;
    let t26035 = t2168 * t1322;
    let t26036 = t2075 * t26035;
    let t26037 = t13472 * t26036;
    let t26040 = t2168 * t1056;
    let t26041 = t5675 * t26040;
    let t26042 = t3937 * t26041;
    let t26045 = t5670 * t26040;
    let t26046 = t13504 * t26045;
    let t26049 = t5601 * t2168;
    let t26050 = t20067 * t26049;
    let t26053 = t7740 * t1322;
    let t26054 = t3937 * t26053;
    let t26057 = t6183 * t25441;
    let t26060 = t20053 * t25450;
    (t26030, t26035, t26036, t26037, t26041, t26042, t26045, t26046, t26050, t26054, t26057, t26060)
}
