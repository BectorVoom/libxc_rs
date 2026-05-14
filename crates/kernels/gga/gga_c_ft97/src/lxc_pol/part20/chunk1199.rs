//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1199/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1199<F: Float>(t28719: F, t317: F, t2842: F, t7091: F, t2844: F, t111732: F, t111807: F, t111815: F, t112376: F, t1466: F, t14678: F, t14914: F, t1506: F, t193: F, t2409: F, t25412: F, t25414: F, t2665: F, t2892: F, t28985: F, t29024: F, t29042: F, t29410: F, t6210: F, t6216: F, t684: F, t6970: F, t7022: F, t798: F, t830: F, t98318: F) -> (F, F) {
    let t112384 = t28719 * t317;
    let t112390 = t7091 * t2842;
    let t112391 = t112390 * t2844;
    let t112393 = -2.0 / 9.0 * t6216 * t25412 * t6970 * t2409 + 2.0 / 9.0 * t6216 * t111807 * t25414 + t6216 * t2665 * t29024 * t2409 / 9.0 - 2.0 / 3.0 * t111732 * t111815 * t14678 + t1466 * t193 * t7022 * t2892 / 6.0 - t14914 * t1506 - 2.0 * t830 * t29410 + t6216 * t2665 * t28985 * t2409 / 9.0 + t1466 * t193 * t798 * t112376 * t317 / 6.0 + t6210 * t29042 / 3.0 - t6216 * t2665 * t112384 * t684 / 9.0 - 4.0 / 81.0 * t98318 + 4.0 * t112391;
    (t112391, t112393)
}
