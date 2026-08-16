//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1297/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1297(t14001: f64, t14466: f64, t3959: f64, t9328: f64, t2409: f64, t26655: f64, t3965: f64, t14765: f64, t3074: f64, t4395: f64, t2362: f64, t1113: f64, t28947: f64, t3972: f64, t3975: f64) -> (f64, f64, f64, f64, f64) {
    let t54566 = t14001 * t14466;
    let t54572 = t3959 * t9328;
    let t54575 = t3965 * t2409 * t26655;
    let t54580 = t3074 * t4395 * t14765;
    let t54581 = t54580 * t2362;
    let t54588 = t3972 * t3975 * t1113 * t28947;
    (t54566, t54572, t54575, t54581, t54588)
}
