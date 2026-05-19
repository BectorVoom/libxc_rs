//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1257/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1257<F: Float>(t40341: F, t40345: F, t37444: F, t39069: F, t39071: F, t39072: F, t40331: F, t40334: F, t41294: F, t41296: F, t41300: F, t41305: F, t41308: F, t41311: F, t41314: F) -> F {
    let t42196 = F::cast_from(0.1440846329149835838e-2_f64) * t40341;
    let t42197 = F::cast_from(0.20496175532535769482e-3_f64) * t40345;
    let t42199 = -t41294 - t41296 + F::cast_from(0.162600798888400151e-2_f64) * t40331 - F::cast_from(0.39032073591371545778e-3_f64) * t40334 + t41300 + t39069 + t42196 - t42197 + t41305 + t41308 - t41311 + F::cast_from(0.12195059916630011325e-2_f64) * t37444 - t39071 - t39072 + t41314;
    t42199
}
