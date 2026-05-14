//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1190/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1190<F: Float>(t2287: F, t164: F, t2053: F, t22666: F, t22671: F, t2281: F, t2288: F, t2289: F, t2292: F, t2293: F, t2295: F, t23059: F, t259: F, t268: F, t270: F, t359: F, t6044: F, t6100: F, t6809: F, t6821: F, t6828: F, t6831: F, t6835: F, t6836: F, t6839: F, t6842: F, t6843: F, t6845: F, t6849: F, t6855: F, t6860: F, t784: F, t864: F) -> (F,) {
    let t23174 = t2287 * t2287;
    let t23179 = 0.5690037894500362222e-1 * t6809 * t2292 * t2295 + 12.0 / t2287 / t2053 * t22671 * t864 + 24.0 * t23059 * t864 + 3.0 * t2288 * t22666 * t864 + 36.0 * t6835 * t22671 * t864 + 0.62590416839503984441e0 * t6821 * t6849 - 0.31295208419751992221e0 * t6831 * t6849 + 0.12141199215318793217e-1 * t2281 * t6842 * t6845 + 0.5690037894500362222e-1 * t6836 * t2292 * t2295 - 0.12141199215318793217e-1 * t2289 * t6842 * t6845 + 0.31295208419751992221e0 * t6839 * t6849 - 0.89035460912337816922e-1 * t6843 * t268 * t270 * t6860 + 0.97362870639228420241e0 * t2293 * t259 * t359 * t6100 + 4.0 * t2288 * t6044 * t6828 + 0.70212178409271598945e-3 * t6855 * t784 * t164 + 1.0 / t23174 * t22671 * t864;
    (t23179,)
}
