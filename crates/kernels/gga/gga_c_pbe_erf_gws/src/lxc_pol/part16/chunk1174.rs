//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1174/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1174<F: Float>(t4227: F, t6781: F, t829: F, t830: F, t14886: F, t4386: F, t892: F, t15036: F, t19906: F, t54463: F, t14935: F, t15021: F, t15081: F, t2376: F, t2408: F, t2409: F, t29751: F, t3066: F, t51881: F, t51896: F, t52194: F, t52551: F, t54461: F, t54465: F, t54473: F, t54484: F, t6793: F, t810: F, t827: F, t8734: F, t8793: F) -> (F,) {
    let t55762 = t6781 * t4227;
    let t55764 = t829 * t830 * t55762;
    let t55769 = t4386 * t892 * t14886;
    let t55773 = 7.0 / 72.0 * t19906 * t15036;
    let t55781 = 7.0 / 576.0 * t54463;
    let t55795 = -t827 * t55764 / 48.0 + 7.0 / 72.0 * t51881 + t6793 * t55769 / 24.0 - t55773 + t8793 * t52194 / 24.0 - 7.0 / 1152.0 * t51896 - t2408 * t29751 * t15021 / 12.0 + t54461 / 1536.0 - t55781 + t54465 / 24.0 - t54473 / 192.0 + 7.0 / 144.0 * t52551 + t54484 / 12.0 + t2408 * t2409 * t2376 * t15081 * t810 / 24.0 + t3066 * t2409 * t8734 * t14935 / 24.0;
    (t55795,)
}
