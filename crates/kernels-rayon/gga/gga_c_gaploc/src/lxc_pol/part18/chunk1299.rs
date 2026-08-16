//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1299/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1299(t32803: f64, t33331: f64, t33332: f64, t2679: f64, t2963: f64, t9796: f64, t2536: f64, t2925: f64, t2009: f64, t2021: f64, t10821: f64, t23157: f64) -> (f64, f64, f64, f64) {
    let t33335 = 0.13803453343411469884e3_f64 * t33331 * t33332 * t32803;
    let t33337 = t9796 * t2963 * t2679;
    let t33338 = 0.76685851907841499352e0_f64 * t33337;
    let t33348 = t2536 * t2925;
    let t33351 = 0.71500979903700853338e0_f64 * t2021 * t33348 * t2009;
    let t33353 = 0.12423108009070322895e3_f64 * t23157 * t10821;
    (t33335, t33338, t33351, t33353)
}
