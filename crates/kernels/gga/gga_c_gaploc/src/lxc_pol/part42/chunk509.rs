//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 509/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk509<F: Float>(t10628: F, t7289: F, t1841: F, t2576: F, t8878: F, t2536: F, t3487: F, t734: F, t3009: F, t7291: F, t7226: F, t2508: F, t2541: F, t8682: F, t3437: F, t731: F) -> (F, F, F, F, F, F, F, F) {
    let t10629 = t7289 * t10628;
    let t10631 = 0.17090058289204942852e-2 * t1841 * t10629;
    let t10632 = t8878 * t2576;
    let t10634 = 0.25635087433807414279e-2 * t1841 * t10632;
    let t10635 = t2536 * t3487;
    let t10636 = t10635 * t734;
    let t10638 = 0.85450291446024714263e-3 * t1841 * t10636;
    let t10639 = t3009 * t7291;
    let t10640 = t7226 * t10639;
    let t10642 = 0.46143157380853345701e-1 * t2508 * t10640;
    let t10643 = t2541 * t8682;
    let t10645 = 0.53833683610995569986e-1 * t2508 * t10643;
    let t10646 = t731 * t3437;
    (t10631, t10634, t10635, t10638, t10639, t10642, t10645, t10646)
}
