//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 954/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk954(t5: f64, t10309: f64, t33358: f64, t38: f64, t8911: f64, t2247: f64, t7574: f64, t8441: f64, t8621: f64, t32132: f64, t32138: f64, t32145: f64, t32156: f64, t8737: f64, t8913: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t33359 = t10309 * t33358;
    let t33362 = t38 * t8911;
    let t33363 = t2247 * t33362;
    let t33367 = t8621 * t8441 * t7574;
    let t33370 = t2247 * t33358;
    let t33374 = piecewise3(t8, 0.0_f64, 5.0_f64 / 144.0_f64 * t32132 * t8913 - 5.0_f64 / 24.0_f64 * t33359 * t32138 - 5.0_f64 / 36.0_f64 * t33363 * t32145 + 5.0_f64 / 72.0_f64 * t8737 * t33367 + 5.0_f64 / 72.0_f64 * t33370 * t32156);
    (t33359, t33362, t33363, t33367, t33370, t33374)
}
