//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1205/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1205<F: Float>(t39437: F, t39440: F, t39443: F, t39445: F, t39420: F, t39422: F, t39424: F, t39426: F, t39429: F, t39431: F, t39434: F, t39448: F) -> F {
    let t41384 = F::new(0.95219938395347901946e-2) * t39437;
    let t41385 = F::new(0.19043987679069580389e-1) * t39440;
    let t41386 = F::new(0.28565981518604370584e-1) * t39443;
    let t41387 = F::new(0.95219938395347901946e-2) * t39445;
    let t41389 = -F::new(0.51220160311720645766e0) * t39420 + F::new(0.10975748638225852664e0) * t39422 + F::new(0.21951497276451705328e0) * t39424 - F::new(0.54878743191129263322e-1) * t39426 + F::new(0.31147743054556651237e-1) * t39429 + F::new(0.10975748638225852664e0) * t39431 + F::new(0.17336443480108537126e0) * t39434 + t41384 - t41385 - t41386 + t41387 - F::new(0.20803732176130244552e1) * t39448;
    t41389
}
