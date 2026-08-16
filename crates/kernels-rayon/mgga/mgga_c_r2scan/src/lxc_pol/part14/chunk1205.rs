//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1205/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1205(t39437: f64, t39440: f64, t39443: f64, t39445: f64, t39420: f64, t39422: f64, t39424: f64, t39426: f64, t39429: f64, t39431: f64, t39434: f64, t39448: f64) -> f64 {
    let t41384 = 0.95219938395347901946e-2_f64 * t39437;
    let t41385 = 0.19043987679069580389e-1_f64 * t39440;
    let t41386 = 0.28565981518604370584e-1_f64 * t39443;
    let t41387 = 0.95219938395347901946e-2_f64 * t39445;
    let t41389 = -0.51220160311720645766e0_f64 * t39420 + 0.10975748638225852664e0_f64 * t39422 + 0.21951497276451705328e0_f64 * t39424 - 0.54878743191129263322e-1_f64 * t39426 + 0.31147743054556651237e-1_f64 * t39429 + 0.10975748638225852664e0_f64 * t39431 + 0.17336443480108537126e0_f64 * t39434 + t41384 - t41385 - t41386 + t41387 - 0.20803732176130244552e1_f64 * t39448;
    t41389
}
