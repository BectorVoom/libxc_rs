//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1413/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1413<F: Float>(t2832: F, t11054: F, t892: F, t11084: F, t1940: F, t198: F, t207: F, t2394: F, t2403: F, t2411: F, t40076: F, t40079: F, t40190: F, t40194: F, t40198: F, t40202: F, t40204: F, t40206: F, t40209: F, t40212: F, t4541: F, t775: F, t890: F) -> F {
    let t41192 = t2832 * t2832;
    let t41197 = t11054 * t892;
    let t41208 = -F::new(4.0) * t11054 * t1940 * t2411 * t890 - F::new(3.0) * t198 * t207 * t2411 * t41192 - F::new(36.0) * t11084 * t2394 * t4541 + F::new(12.0) * t2403 * t41197 * t775 + t40076 - t40079 + t40190 + t40194 + t40198 + t40202 + t40204 - t40206 + t40209 + t40212;
    t41208
}
