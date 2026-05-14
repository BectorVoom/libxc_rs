//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1038/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1038<F: Float>(t1364: F, t20852: F, t3619: F, t5658: F, t14140: F, t2191: F, t3593: F, t2192: F, t3278: F, t2181: F, t1216: F, t1349: F, t13966: F, t1402: F, t14093: F, t14100: F, t14116: F, t14118: F, t14120: F, t20832: F, t20835: F, t20839: F, t20843: F, t20845: F, t20846: F, t20849: F, t3819: F, t3857: F, t417: F, t425: F, t5798: F, t5864: F) -> (F,) {
    let t20853 = t20852 * t1364;
    let t20856 = t5658 * t3619;
    let t20859 = t14140 * t2191;
    let t20860 = t20859 * t3593;
    let t20863 = t2192 * t3278;
    let t20866 = t2181 * t3278;
    let t20878 = 0.18741227171104716398e-1 * t14100 * t20832 + 0.46853067927761790996e-2 * t1349 * t20835 + 0.18741227171104716398e-1 * t3857 * t20839 - t20843 - t20845 + 0.93706135855523581992e-2 * t1349 * t20846 + 0.46853067927761790996e-2 * t1349 * t20849 + 0.28111840756657074598e-1 * t417 * t20853 + 0.14055920378328537299e-1 * t417 * t20856 - 0.56223681513314149196e-1 * t417 * t20860 - 0.93706135855523581992e-2 * t3819 * t20863 - 0.14055920378328537299e-1 * t14093 * t20866 - 2.0 * t5798 * t1402 - 2.0 * t1216 * t5864 - 0.46853067927761790996e-2 * t14116 - 0.14055920378328537299e-1 * t14120 - 0.93706135855523581992e-2 * t14118 - 0.46853067927761790996e-2 * t3857 * t425 - t13966;
    (t20878,)
}
