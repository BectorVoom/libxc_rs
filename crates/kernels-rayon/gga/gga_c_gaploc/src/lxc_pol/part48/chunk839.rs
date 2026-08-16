//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 839/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk839(t42651: f64, t2321: f64, t38019: f64, t9074: f64, t42673: f64, t1063: f64, t35908: f64, t894: f64, t13304: f64, t2312: f64, t13296: f64, t158: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44413 = 0.28455006635676149599e-1_f64 * t42651;
    let t44415 = t9074 * t38019 * t2321;
    let t44416 = 0.11856252764865062333e-2_f64 * t44415;
    let t44420 = 0.63233348079280332443e-2_f64 * t42673;
    let t44423 = 0.28455006635676149599e-1_f64 * t1063 * t894 * t35908;
    let t44424 = t2312 * t13304;
    let t44425 = 0.11856252764865062333e-2_f64 * t44424;
    let t44426 = t158 * t13296;
    (t44413, t44416, t44420, t44423, t44425, t44426)
}
