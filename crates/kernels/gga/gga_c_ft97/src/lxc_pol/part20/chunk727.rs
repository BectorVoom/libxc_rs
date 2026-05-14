//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 727/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk727<F: Float>(t10683: F, t15477: F, t319: F, t14603: F, t296: F, t1248: F, t2682: F, t2862: F, t871: F, t1882: F, t4188: F, t1255: F, t2409: F, t835: F, t10735: F, t10745: F, t10749: F, t10750: F, t10752: F, t15455: F, t15463: F, t15467: F, t15471: F, t15474: F, t1901: F, t446: F) -> (F, F) {
    let t15479 = t10683 * t319 * t15477;
    let t15482 = t296 * t14603;
    let t15485 = t1248 * t2682;
    let t15487 = t2862 * t871 * t15485;
    let t15491 = 2.0 / 27.0 * t1882 * t4188;
    let t15493 = t835 * t1255 * t2409;
    let t15496 = 2.0 / 9.0 * t1901 * t15455 - 8.0 / 27.0 * t10735 - t10745 / 9.0 - t10749 - 4.0 / 3.0 * t1901 * t15463 - t15467 + t10750 / 9.0 + 2.0 / 9.0 * t10752 + t15471 - 2.0 / 3.0 * t446 * t15474 - 2.0 * t446 * t15479 - 2.0 * t446 * t15482 - 2.0 / 3.0 * t446 * t15487 + t15491 + 2.0 / 9.0 * t446 * t15493;
    (t15485, t15496)
}
