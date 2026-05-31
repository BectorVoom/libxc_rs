//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 829/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk829<F: Float>(t13017: F, t3439: F, t2075: F, t920: F, t2222: F, t2221: F, t2157: F, t3578: F, t144: F, t11593: F, t13000: F, t13004: F, t13007: F, t13010: F, t13014: F, t1901: F, t446: F, t9270: F, t9272: F, t9274: F, t9282: F, t9298: F, t9300: F, t9302: F) -> (F, F) {
    let t13018 = t3439 * t13017;
    let t13021 = t920 * t2075;
    let t13022 = t2222 * t13021;
    let t13023 = t2221 * t13022;
    let t13030 = t3578 * t2157;
    let t13031 = t144 * t13030;
    let t13037 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t11593 * t13000 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t11593 * t13004 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t13007 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t13010 + t1901 * t13014 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t13018 + t1901 * t13023 / F::cast_from(9.0_f64) - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9270 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9272 + t9274 / F::cast_from(9.0_f64) - t9282 / F::cast_from(9.0_f64) - t446 * t13031 / F::cast_from(3.0_f64) - F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t9298 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9300 + F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t9302;
    (t13030, t13037)
}
