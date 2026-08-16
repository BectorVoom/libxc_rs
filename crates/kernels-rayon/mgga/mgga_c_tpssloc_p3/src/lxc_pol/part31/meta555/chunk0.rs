//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1783/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1783(t81153: f64, t81317: f64, t81398: f64, t531: f64, t7216: f64, t2056: f64, t40772: f64, t193: f64, t201: f64, t7109: f64, t10143: f64, t82069: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t84597 = 0.19739208802178717238e0_f64 * t81153;
    let t84659 = 0.55440370401180965083e0_f64 * t81317;
    let t84705 = 0.27415567780803773942e-2_f64 * t81398;
    let t84733 = t531 * t7216;
    let t84766 = t2056 * t40772;
    let t84797 = t193 * t201 * t7109;
    let t84800 = t7109 * t10143;
    let t84820 = 0.19739208802178717238e0_f64 * t82069;
    (t84597, t84659, t84705, t84733, t84766, t84797, t84800, t84820)
}
