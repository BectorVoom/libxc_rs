//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3130/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3130(t5245: f64, t5819: f64, t81128: f64, t81130: f64, t81132: f64, t81134: f64, t81136: f64, t81138: f64, t81145: f64, t81148: f64, t81150: f64, t81152: f64, t81254: f64, t81257: f64, t81259: f64, t81261: f64, t81264: f64, t81266: f64, t81307: f64, t81309: f64, t81313: f64, t81315: f64) -> (f64, f64) {
    let t82368 = t5819 * t5245;
    let t82385 = t81128 + t81130 + t81132 + t81134 + t81136 - t81138 - t81145 + t81148 - t81150 + t81152 + t81254 - t81257 - t81259 + t81261 + t81264 - t81266 - t81307 + t81309 - t81313 - t81315;
    (t82368, t82385)
}
