//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1132/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1132(t4260: f64, t883: f64, t6490: f64, t6525: f64, t1436: f64, t9544: f64, t1538: f64, t20395: f64, t6583: f64, t20481: f64, t21414: f64, t123: f64, t6393: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30204 = t4260 * t883;
    let t30207 = 0.94850022118920498664e-2_f64 * t6525 * t30204 * t6490;
    let t30246 = t1436 * t9544;
    let t30247 = 0.1022478025437886658e1_f64 * t30246;
    let t30250 = t6583 * t1538 * t883 * t20395;
    let t30251 = 0.76685851907841499352e0_f64 * t30250;
    let t30253 = 0.59584149919750711116e-1_f64 * t20481 * t21414;
    let t30258 = t6393 * t123 * t883;
    (t30204, t30207, t30247, t30251, t30253, t30258)
}
