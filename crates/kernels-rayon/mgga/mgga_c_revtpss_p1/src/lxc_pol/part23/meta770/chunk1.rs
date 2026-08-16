//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2572/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2572(t57270: f64, t1222: f64, t5368: f64, t697: f64, t3625: f64, t44250: f64, t5406: f64, t3781: f64, t5219: f64, t5330: f64, t12881: f64, t5391: f64) -> (f64, f64, f64, f64, f64) {
    let t57271 = t57270 / 162.0_f64;
    let t57273 = t1222 * t697 * t5368;
    let t57274 = t57273 / 432.0_f64;
    let t57331 = t3625 * t44250 * t5406;
    let t57382 = t5219 * t3781 * t5330;
    let t57421 = t5391 * t12881;
    (t57271, t57274, t57331, t57382, t57421)
}
