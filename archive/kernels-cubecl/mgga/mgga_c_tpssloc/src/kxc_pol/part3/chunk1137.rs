//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1137/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1137<F: Float>(t11219: F, t14726: F, t136: F, t4775: F, t699: F, t14736: F, t3297: F, t14740: F, t14731: F, t1113: F, t14749: F, t14753: F) -> (F, F, F, F, F, F, F, F) {
    let t14778 = t11219 * t14726;
    let t14779 = t136 * t14778;
    let t14781 = t699 * t4775;
    let t14782 = F::cast_from(0.22076e0_f64) * t14781;
    let t14783 = t3297 * t14736;
    let t14784 = t136 * t14783;
    let t14786 = t3297 * t14740;
    let t14787 = t136 * t14786;
    let t14789 = t3297 * t14731;
    let t14790 = t136 * t14789;
    let t14792 = t1113 * t14749;
    let t14793 = t136 * t14792;
    let t14795 = t1113 * t14753;
    (t14779, t14781, t14782, t14784, t14787, t14790, t14793, t14795)
}
