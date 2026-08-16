//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 647/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk647(t1628: f64, t3714: f64, t1: f64, t12012: f64, t544: f64, t10361: f64, t10363: f64, t10367: f64, t10369: f64, t10373: f64, t10377: f64, t10381: f64, t10384: f64, t10387: f64, t10394: f64, t1424: f64, t597: f64, t9362: f64, t9365: f64, t9369: f64) -> (f64, f64, f64) {
    let t12075 = t1628 * t3714;
    let t12078 = t12012 * t1;
    let t12079 = t544 * t12078;
    let t12085 = -t10361 - t10363 - t10367 - t10369 - t10373 - t10377 + t10381 + t10384 + t10387 + 0.30674340763136599741e1_f64 * t597 * t12075 - 0.39722766613167140743e-1_f64 * t12079 * t1424 + 0.38342925953920749677e0_f64 * t9362 + 0.38342925953920749677e0_f64 * t9365 - 0.85206502119823888171e-1_f64 * t9369 + t10394;
    (t12078, t12079, t12085)
}
