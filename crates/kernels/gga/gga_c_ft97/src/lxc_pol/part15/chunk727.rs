//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 727/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk727<F: Float>(t13976: F, t13981: F, t18241: F, t21402: F, t21414: F, t21419: F, t21422: F, t21433: F, t21437: F, t21448: F, t21556: F, t21567: F, t21626: F, t18265: F, t18266: F, t18381: F, t18382: F, t18383: F, t21406: F, t21410: F, t21440: F, t21444: F, t21451: F, t21455: F, t21459: F, t9972: F) -> (F, F) {
    let t21708 = -t18241 - t21402 / 3.0 - 2.0 * t21419 - t21556 / 4.0 - t21414 / 9.0 + 2.0 * t21422 - 10.0 / 81.0 * t21433 - 2.0 / 3.0 * t21437 + 4.0 / 9.0 * t21448 - t13976 - t13981 + t21567 / 8.0 + t21626 / 6.0;
    let t21716 = -t18265 + t18266 - 2.0 / 3.0 * t21406 - 2.0 / 3.0 * t21410 + t21459 / 3.0 + 2.0 / 3.0 * t21440 + 2.0 / 9.0 * t21444 - 2.0 / 9.0 * t21451 + t21455 / 3.0 + t18381 - t18382 + t18383 - t9972;
    (t21708, t21716)
}
