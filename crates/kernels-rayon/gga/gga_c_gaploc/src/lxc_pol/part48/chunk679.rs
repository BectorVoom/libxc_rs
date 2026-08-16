//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 679/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk679(t12993: f64, t2487: f64, t10268: f64, t2365: f64, t4391: f64, t3005: f64, t3295: f64, t9800: f64, t11053: f64, t9805: f64, t1029: f64, t9796: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12994 = t2487 * t12993;
    let t12996 = t2365 * t10268;
    let t12997 = t4391 * t12996;
    let t13052 = t3005 * t3295;
    let t13053 = t9800 * t13052;
    let t13055 = t11053 * t3295;
    let t13056 = t9805 * t13055;
    let t13058 = t1029 * t3295;
    let t13059 = t9796 * t13058;
    (t12994, t12996, t12997, t13052, t13053, t13055, t13056, t13058, t13059)
}
