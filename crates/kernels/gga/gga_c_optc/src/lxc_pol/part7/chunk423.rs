//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 423/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk423<F: Float>(t2051: F, t2052: F, t138: F, t637: F, t120: F, t658: F, t124: F, t1928: F, t1948: F, t121: F, t641: F, t642: F) -> (F, F, F, F, F, F) {
    let t2053 = t2051 + t2052;
    let t2057 = t637 * t138;
    let t2060 = t120 * t658;
    let t2061 = t124 * t1928;
    let t2064 = t124 * t1948;
    let t2067 = -F::cast_from(0.12897460341341234505e3_f64) * t2053 * t121 * t124 + F::cast_from(0.7738476204804740703e3_f64) * t2057 * t642 - F::cast_from(0.15476952409609481406e4_f64) * t2060 * t2061 + F::cast_from(0.38692381024023703515e3_f64) * t641 * t2064;
    (t2053, t2057, t2060, t2061, t2064, t2067)
}
