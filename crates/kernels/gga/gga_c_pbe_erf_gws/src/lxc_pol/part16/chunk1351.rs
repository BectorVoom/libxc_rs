//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1351/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1351<F: Float>(t4227: F, t6781: F, t829: F, t830: F, t14886: F, t4386: F, t892: F, t15036: F, t19906: F, t54463: F, t14935: F, t15021: F, t15081: F, t2376: F, t2408: F, t2409: F, t29751: F, t3066: F, t51881: F, t51896: F, t52194: F, t52551: F, t54461: F, t54465: F, t54473: F, t54484: F, t6793: F, t810: F, t827: F, t8734: F, t8793: F) -> F {
    let t55762 = t6781 * t4227;
    let t55764 = t829 * t830 * t55762;
    let t55769 = t4386 * t892 * t14886;
    let t55773 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t19906 * t15036;
    let t55781 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t54463;
    let t55795 = -t827 * t55764 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t51881 + t6793 * t55769 / F::cast_from(24.0_f64) - t55773 + t8793 * t52194 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t51896 - t2408 * t29751 * t15021 / F::cast_from(12.0_f64) + t54461 / F::cast_from(1536.0_f64) - t55781 + t54465 / F::cast_from(24.0_f64) - t54473 / F::cast_from(192.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t52551 + t54484 / F::cast_from(12.0_f64) + t2408 * t2409 * t2376 * t15081 * t810 / F::cast_from(24.0_f64) + t3066 * t2409 * t8734 * t14935 / F::cast_from(24.0_f64);
    t55795
}
