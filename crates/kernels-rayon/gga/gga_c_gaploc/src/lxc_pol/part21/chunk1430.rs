//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1430/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1430(t38974: f64, t701: f64, t1457: f64, t2004: f64, t28585: f64, t33315: f64, t33317: f64, t33319: f64, t33321: f64, t33325: f64, t33328: f64, t33335: f64, t33338: f64, t33351: f64, t33353: f64, t33356: f64, t33359: f64, t33363: f64, t33365: f64) -> (f64, f64) {
    let t39107 = t38974 * t701;
    let t39111 = t33315 + t33317 + t33319 + t33321 - t33325 + t33328 - t33335 - t33338 - t28585 + 0.71500979903700853338e0_f64 * t2004 * t1457 * t39107 - t33351 - t33353 + t33356 - t33359 - t33363 - t33365;
    (t39107, t39111)
}
