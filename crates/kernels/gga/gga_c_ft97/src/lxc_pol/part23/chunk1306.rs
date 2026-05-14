//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1306/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1306<F: Float>(t28963: F, t6963: F, t111657: F, t111664: F, t113581: F, t1466: F, t193: F, t19872: F, t25412: F, t25413: F, t25459: F, t25465: F, t2665: F, t28835: F, t28966: F, t29000: F, t29416: F, t31348: F, t31665: F, t31683: F, t31963: F, t4973: F, t6210: F, t6216: F, t6267: F, t6970: F, t7024: F, t98257: F) -> (F,) {
    let t125500 = t6963 * t28963;
    let t125522 = t111657 - 4.0 / 9.0 * t29000 * t25412 * t25413 * t19872 + t31963 * t6267 / 6.0 - t98257 + 2.0 / 9.0 * t125500 - 2.0 / 3.0 * t6210 * t31683 - 2.0 / 3.0 * t1466 * t193 * t113581 * t6970 - 2.0 / 3.0 * t1466 * t193 * t28835 * t28966 - t25459 * t31348 / 18.0 - t6216 * t2665 * t25465 * t4973 / 18.0 + t6210 * t31665 / 6.0 + t29416 * t7024 / 3.0 - t111664;
    (t125522,)
}
