//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 950/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk950<F: Float>(t4417: F, t4724: F, t1060: F, t17164: F, t1901: F, t20027: F, t20731: F, t20749: F, t20754: F, t2205: F, t2210: F, t3434: F, t3439: F, t3440: F, t40911: F, t446: F, t49634: F, t49661: F, t63530: F, t63536: F, t63613: F, t76777: F, t85320: F, t925: F) -> (F, F) {
    let t87462 = t4417 * t4724;
    let t87517 = 8.0 / 9.0 * t1901 * t2210 * t3434 * t85320 - 8.0 / 27.0 * t1901 * t3439 * t3440 * t85320 + 8.0 / 9.0 * t1901 * t49634 * t20749 + 8.0 / 9.0 * t1901 * t17164 * t20754 - 16.0 / 27.0 * t63530 - 8.0 / 9.0 * t63536 + 16.0 / 9.0 * t446 * t2205 * t1060 * t20027 + 4.0 / 9.0 * t76777 + 112.0 / 81.0 * t49661 + 8.0 / 3.0 * t1901 * t40911 * t20731 * t925 + 16.0 / 27.0 * t63613;
    (t87462, t87517)
}
