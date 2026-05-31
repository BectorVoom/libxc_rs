//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 914/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk914<F: Float>(t1544: F, t1583: F, t18875: F, t1940: F, t1963: F, t198: F, t207: F, t2403: F, t25440: F, t25445: F, t27363: F, t27368: F, t27375: F, t27384: F, t4343: F, t4433: F, t4537: F, t4541: F, t7087: F, t7091: F, t775: F, t7783: F, t890: F, t892: F) -> F {
    let t27754 = t198 * t207 * t27363 * t892 + F::cast_from(3.0_f64) * t1544 * t2403 * t7087 - t1583 * t1940 * t25440 - F::cast_from(3.0_f64) * t18875 * t2403 * t7091 + F::cast_from(2.0_f64) * t1940 * t25445 * t27384 - t1940 * t27368 * t890 - t1940 * t4537 * t7091 + F::cast_from(3.0_f64) * t1963 * t2403 * t4343 + F::cast_from(6.0_f64) * t1963 * t4433 * t4541 - F::cast_from(3.0_f64) * t2403 * t27375 * t7091 + F::cast_from(3.0_f64) * t2403 * t775 * t7783;
    t27754
}
