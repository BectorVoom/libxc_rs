//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 987/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk987<F: Float>(t38164: F, t38175: F, t38189: F, t11450: F, t3270: F, t1115: F, t1563: F, t36967: F, t1234: F, t1543: F, t11449: F, t1561: F, t14402: F, t795: F, t498: F, t11002: F, t2259: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t38657 = 0.51410067763503603055e-4 * t38164;
    let t38661 = 0.34909953929791734801e0 * t38175;
    let t38666 = 0.46160609703545424213e1 * t38189;
    let t38678 = t3270 * t11450;
    let t38688 = t36967 * t1115 * t1563;
    let t38697 = t3270 * t1115 * t1234;
    let t38715 = t3270 * t1115 * t1543;
    let t38718 = t1561 * t11449;
    let t38722 = t14402 * t795;
    let t38723 = t3270 * t38722;
    let t38739 = t498 * t11449;
    let t38749 = t11002 * t1115 * t2259;
    (t38657, t38661, t38666, t38678, t38688, t38697, t38715, t38718, t38723, t38739, t38749)
}
