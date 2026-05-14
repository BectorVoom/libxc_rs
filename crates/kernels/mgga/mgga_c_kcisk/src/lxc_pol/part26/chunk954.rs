//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 954/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk954<F: Float>(t1364: F, t25947: F, t2181: F, t220: F, t5646: F, t5703: F, t20838: F, t2191: F, t1056: F, t8105: F, t1349: F, t13966: F, t14093: F, t14100: F, t20825: F, t20827: F, t20843: F, t20845: F, t2209: F, t25922: F, t25925: F, t25927: F, t25931: F, t25934: F, t25937: F, t25941: F, t25944: F, t3819: F, t3857: F, t417: F, t5798: F) -> (F,) {
    let t25948 = t25947 * t1364;
    let t25951 = t2181 * t220;
    let t25954 = t5646 * t5703;
    let t25957 = t20838 * t2191;
    let t25960 = t8105 * t1056;
    let t25965 = -t20825 - t20827 - t20843 - t20845 + 0.46853067927761790996e-2 * t1349 * t25922 - 0.93706135855523581992e-2 * t25925 - 0.14055920378328537299e-1 * t1349 * t25927 - 0.56223681513314149196e-1 * t417 * t25931 + 0.28111840756657074598e-1 * t417 * t25934 + 0.46853067927761790996e-2 * t1349 * t25937 + 0.14055920378328537299e-1 * t417 * t25941 - 0.14055920378328537299e-1 * t14093 * t25944 - 0.93706135855523581992e-2 * t3819 * t25948 + 0.18741227171104716398e-1 * t14100 * t25951 + 0.93706135855523581992e-2 * t1349 * t25954 + 0.18741227171104716398e-1 * t3857 * t25957 + 0.46853067927761790996e-2 * t3819 * t25960 - 2.0 * t5798 * t2209 - t13966;
    (t25965,)
}
