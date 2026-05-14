//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1413/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1413<F: Float>(t1466: F, t31678: F, t681: F, t25462: F, t31344: F, t31664: F, t112549: F, t112565: F, t112568: F, t126951: F, t1506: F, t18987: F, t19240: F, t193: F, t24964: F, t31677: F, t317: F, t31963: F, t4309: F, t5299: F, t6222: F, t6223: F, t6225: F, t7022: F, t880: F, t98429: F) -> (F,) {
    let t128606 = t1466 * t681 * t31678;
    let t128608 = t25462 * t31344;
    let t128617 = t1466 * t681 * t31664;
    let t128634 = 2.0 / 27.0 * t98429 - t1466 * t193 * t126951 * t6223 / 3.0 + t128606 / 9.0 + t128608 / 27.0 + t1466 * t193 * t7022 * t4309 / 3.0 - t112549 + t112565 + t112568 - t31963 * t6225 / 3.0 - t128617 / 18.0 - t1466 * t193 * t24964 * t31677 / 3.0 - t1466 * t193 * t6222 * t880 * t5299 / 3.0 - t1466 * t193 * t6222 * t317 * t19240 / 3.0 - t18987 * t1506;
    (t128634,)
}
