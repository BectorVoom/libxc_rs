//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1086/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1086<F: Float>(t81963: F, t81993: F, t13598: F, t1526: F, t21911: F, t5213: F, t9483: F, t21922: F, t21918: F, t13616: F, t16579: F, t18987: F, t19020: F, t19030: F, t19263: F, t19267: F, t19271: F, t19308: F, t2320: F, t2639: F, t3806: F, t44663: F, t44709: F, t44716: F, t72910: F, t72912: F, t72950: F) -> (F, F) {
    let t81994 = t81963 + t81993;
    let t82488 = t1526 * t13598 * t21911;
    let t82491 = t1526 * t9483 * t5213;
    let t82494 = t1526 * t9483 * t21922;
    let t82497 = t1526 * t9483 * t21918;
    let t82518 = t72910 - t72912 + t44663 / 54.0 + t44709 / 18.0 - t44716 + t18987 + t19308 - t82488 / 27.0 - t82491 / 18.0 - t82494 / 36.0 + t82497 / 18.0 + t1526 * t13616 * t19267 / 3.0 + t1526 * t2320 * t19020 / 6.0 - t1526 * t2320 * t19030 / 12.0 - t1526 * t2320 * t2639 * t16579 / 12.0 - t1526 * t3806 * t19271 / 9.0 - t1526 * t2320 * t19263 / 6.0 - t72950;
    (t81994, t82518)
}
