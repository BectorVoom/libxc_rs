//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1117/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1117<F: Float>(t119706: F, t119747: F, t125962: F, t125968: F, t125976: F, t125977: F, t125980: F, t125981: F, t125985: F, t125988: F, t125997: F, t1940: F, t2403: F, t25206: F, t27169: F, t27173: F, t27376: F, t27382: F, t27385: F, t27387: F, t27391: F, t27395: F, t27402: F, t31859: F, t31863: F, t31876: F, t33727: F, t605: F, t7092: F, t7749: F, t8490: F, t8494: F) -> F {
    let t126004 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t8490 * t27395 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t8494 * t27173 + t1940 * t31876 * t27402 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t119747 * t27376 + F::cast_from(2.0_f64) * t27382 * t125962 - t1940 * t31863 * t27387 / F::cast_from(2.0_f64) + t125968 * t27385 + t1940 * t33727 * t605 / F::cast_from(2.0_f64) + t1940 * t31876 * t27391 - t125976 + F::cast_from(3.0_f64) * t119706 * t125977 - F::cast_from(3.0_f64) * t125980 * t125981 - F::cast_from(3.0_f64) * t25206 * t125985 + F::cast_from(3.0_f64) * t119706 * t125988 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t31859 * t7749 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t8494 * t27169 - t1940 * t125997 * t7092 / F::cast_from(2.0_f64) - t1940 * t31863 * t27391 / F::cast_from(2.0_f64);
    t126004
}
