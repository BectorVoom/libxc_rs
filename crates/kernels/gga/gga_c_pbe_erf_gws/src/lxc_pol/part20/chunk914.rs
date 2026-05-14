//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 914/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk914<F: Float>(t3626: F, t751: F, t481: F, t981: F, t5651: F, t3685: F, t475: F, t142: F, t3644: F, t525: F, t2919: F, t524: F, t2037: F, t5601: F, t6036: F, t6039: F, t6043: F, t6049: F, t6050: F, t6053: F, t6058: F, t6061: F, t6064: F, t8497: F, t8502: F, t8503: F, t988: F) -> (F,) {
    let t11290 = t751 * t3626;
    let t11292 = t981 * t481;
    let t11293 = t5651 * t11292;
    let t11296 = t475 * t3685;
    let t11299 = t142 * t3644;
    let t11300 = t525 * t11299;
    let t11303 = t524 * t2919;
    let t11304 = t11303 * t142;
    let t11306 = -t6036 + t8502 + 0.79828226735030727293e-1 * t8503 - 0.18218576931715098443e-4 * t6039 - t6043 + t6049 - 0.53218817823353818195e-1 * t6050 - t6053 - t6058 + 0.39914113367515363646e-1 * t6061 + t6064 + 0.19957056683757681823e-1 * t11290 - 3.0 * t8497 * t11293 + 3.0 * t11296 * t2037 + 6.0 * t5601 * t11300 + t988 * t11304;
    (t11306,)
}
