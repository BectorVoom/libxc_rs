//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1116/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1116(t3775: f64, t9980: f64, t33831: f64, t33834: f64, t33836: f64, t33838: f64, t33840: f64, t33842: f64, t33847: f64, t33850: f64, t33852: f64, t33855: f64) -> f64 {
    let t33857 = t3775 * t9980;
    let t33859 = -0.34752370105806885418e-3_f64 * t33831 + 0.12228868272569444445e-4_f64 * t33834 - 0.2318836277704281739e-4_f64 * t33836 - 0.90579542097823505428e-7_f64 * t33838 + 0.60706991790943943129e-6_f64 * t33840 - 0.10793703140429833089e-5_f64 * t33842 + 0.92386400563397210585e-6_f64 * t33847 - 0.16882049790461501058e-6_f64 * t33850 + 0.27991498566271340012e-7_f64 * t33852 + 0.10110318318802209383e-5_f64 * t33855 - 0.57970906942607043474e-5_f64 * t33857;
    t33859
}
