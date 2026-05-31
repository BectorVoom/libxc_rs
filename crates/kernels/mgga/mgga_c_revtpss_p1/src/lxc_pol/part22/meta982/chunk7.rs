//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3330/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3330<F: Float>(t1544: F, t2411: F, t14365: F, t1583: F, t18392: F, t18865: F, t1940: F, t198: F, t205: F, t2403: F, t2404: F, t2408: F, t2832: F, t40076: F, t40079: F, t40194: F, t40198: F, t41154: F, t6079: F, t61519: F, t62307: F, t62308: F, t62311: F, t62312: F, t765: F) -> F {
    let t63185 = t2411 * t1544;
    let t63186 = t63185 * t14365;
    let t63189 = -F::cast_from(24.0_f64) * t1583 * t198 * t205 * t63186 - F::cast_from(6.0_f64) * t1940 * t2408 * t41154 * t6079 + F::cast_from(6.0_f64) * t18392 * t2403 * t2404 - t18865 * t1940 * t2832 + F::cast_from(3.0_f64) * t198 * t61519 * t765 + t40076 - t40079 + t40194 + t40198 + t62307 - t62308 + t62311 - t62312;
    t63189
}
