//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1095/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1095<F: Float>(t14365: F, t1940: F, t198: F, t207: F, t2071: F, t2394: F, t2403: F, t2408: F, t2430: F, t26580: F, t26585: F, t26590: F, t2832: F, t4541: F, t7428: F, t7432: F, t775: F, t890: F, t892: F) -> F {
    let t26625 = t198 * t207 * t26580 * t892 - F::cast_from(6.0_f64) * t14365 * t2403 * t7432 + F::cast_from(2.0_f64) * t1940 * t2408 * t26590 - F::cast_from(2.0_f64) * t1940 * t26585 * t890 - t1940 * t2832 * t7432 + F::cast_from(6.0_f64) * t2071 * t2394 * t4541 + F::cast_from(3.0_f64) * t2071 * t2403 * t2430 + F::cast_from(6.0_f64) * t2403 * t7428 * t775;
    t26625
}
