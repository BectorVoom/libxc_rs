//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2351/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2351<F: Float>(t13242: F, t1510: F, t16662: F, t16891: F, t16912: F, t20885: F, t20887: F, t20891: F, t232: F, t2643: F, t2645: F, t2647: F, t4180: F, t4181: F, t4234: F, t47277: F, t58495: F, t59251: F, t59255: F, t59257: F, t59259: F, t59261: F, t67607: F, t9642: F) -> F {
    let t68186 = -t47277 + t2643 * t2645 * t16891 * t16912 / F::cast_from(256.0_f64) + t9642 * t20887 / F::cast_from(256.0_f64) + t2643 * t2645 * t13242 * t20885 / F::cast_from(256.0_f64) + t2643 * t2645 * t4181 * t232 * t16662 / F::cast_from(256.0_f64) - t9642 * t20891 / F::cast_from(1024.0_f64) - t2643 * t4180 * t58495 * t1510 / F::cast_from(1024.0_f64) - t2643 * t4180 * t16891 * t4234 / F::cast_from(1024.0_f64) + t2643 * t2645 * t67607 * t2647 / F::cast_from(768.0_f64) - F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t59251 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t59255 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t59257 - F::cast_from(119.0_f64) / F::cast_from(576.0_f64) * t59259 + F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t59261;
    t68186
}
