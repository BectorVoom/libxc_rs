//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1136/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1136(t1235: f64, t5362: f64, t1219: f64, t1778: f64, t1225: f64, t4186: f64, t1012: f64, t1222: f64, t3657: f64, t3658: f64, t3679: f64, t3684: f64, t3718: f64, t5340: f64, t5343: f64, t5348: f64, t5354: f64, t5358: f64) -> (f64, f64, f64, f64, f64) {
    let t5363 = t1235 * t5362;
    let t5366 = t1778 * t1219;
    let t5368 = t1225 * t4186;
    let t5369 = t1012 * t5368;
    let t5372 = 0.42874018118069736972e-3_f64 * t5340 * t5343 - 0.21437009059034868486e-3_f64 * t3718 * t5348 - 0.21437009059034868486e-3_f64 * t3718 * t5354 - t5358 / 864.0_f64 - t3657 + 0.14291339372689912324e-3_f64 * t3658 - 0.14291339372689912324e-3_f64 * t5363 - 0.14291339372689912324e-3_f64 * t3679 - t5366 / 108.0_f64 - t3684 - t1222 * t5369 / 288.0_f64;
    (t5363, t5366, t5368, t5369, t5372)
}
