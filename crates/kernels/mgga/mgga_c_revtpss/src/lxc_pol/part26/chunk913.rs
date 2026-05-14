//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 913/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk913<F: Float>(t12766: F, t13164: F, t1277: F, t13107: F, t225: F, t494: F, t1214: F, t3738: F, t3737: F, t1269: F, t3555: F, t1275: F, t1294: F, t1204: F, t1210: F, t1215: F, t12666: F, t12673: F, t12690: F, t12696: F, t1271: F, t1274: F, t1295: F, t3552: F, t3556: F, t3561: F, t3585: F, t3729: F, t3732: F, t3739: F, t3791: F, t460: F, t495: F) -> (F,) {
    let t13165 = t12766 + t13164;
    let t13166 = t1277 * t13165;
    let t13170 = t13107 * t225 * t494;
    let t13173 = t1214 * t3738;
    let t13174 = t3737 * t13173;
    let t13177 = t3555 * t1269;
    let t13180 = t1275 * t1275;
    let t13181 = 1.0 / t13180;
    let t13182 = t225 * t13181;
    let t13183 = t3738 * t1294;
    let t13184 = t13182 * t13183;
    let t13189 = 0.39512695097613069591e1 * t3732 * t3739 - 0.19756347548806534796e1 * t12666 * t1215 - 0.19756347548806534796e1 * t3556 * t3585 + 0.39512695097613069591e1 * t3561 * t3739 - 0.19756347548806534796e1 * t12673 * t1295 - 0.19756347548806534796e1 * t3561 * t3791 + 0.65854491829355115987e0 * t12690 * t495 + 0.19756347548806534796e1 * t3552 * t1271 + 0.39512695097613069591e1 * t1274 * t12696 - 0.65854491829355115987e0 * t1274 * t13166 + 0.65854491829355115987e0 * t460 * t13170 - 0.39512695097613069591e1 * t1210 * t13174 - 0.39512695097613069591e1 * t13177 * t1215 - 0.39512695097613069591e1 * t1274 * t13184 + 0.19756347548806534796e1 * t1204 * t3729;
    (t13189,)
}
