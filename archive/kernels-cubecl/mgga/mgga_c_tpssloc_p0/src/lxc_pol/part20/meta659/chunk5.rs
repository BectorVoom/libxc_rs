//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2457/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2457<F: Float>(t10903: F, t14507: F, t14651: F, t3069: F, t10394: F, t10403: F, t10937: F, t10952: F, t13995: F, t14069: F, t14077: F, t14103: F, t14211: F, t1622: F, t3071: F, t3073: F, t3120: F, t3123: F, t3134: F, t42746: F, t43262: F, t43273: F, t43277: F, t43281: F, t43285: F, t607: F, t883: F) -> F {
    let t50302 = t14507 * t10903;
    let t50324 = t14651 * t3069;
    let t50329 = -t50302 * t3134 / F::cast_from(96.0_f64) + t42746 * t1622 / F::cast_from(4608.0_f64) + t10403 * t3071 * t14211 * t3120 * t883 * t607 / F::cast_from(768.0_f64) - F::cast_from(209.0_f64) / F::cast_from(3888.0_f64) * t43262 + t43273 / F::cast_from(144.0_f64) + t43277 / F::cast_from(768.0_f64) - t43281 / F::cast_from(768.0_f64) + t43285 / F::cast_from(4608.0_f64) - t14077 * t3123 / F::cast_from(192.0_f64) - t10952 * t14103 / F::cast_from(1024.0_f64) - t10937 * t14069 / F::cast_from(144.0_f64) + t50324 * t3073 / F::cast_from(768.0_f64) + t13995 * t10394 / F::cast_from(1536.0_f64);
    t50329
}
