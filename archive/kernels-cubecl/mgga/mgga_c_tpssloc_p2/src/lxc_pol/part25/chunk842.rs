//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 842/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk842<F: Float>(t10658: F, t10856: F, t360: F, t1021: F, t248: F, t1004: F, t3047: F, t3053: F, t3117: F, t1043: F, t676: F, t884: F) -> (F, F, F, F, F) {
    let t10857 = t10658 + t10856;
    let t10858 = t10857 * t360;
    let t10860 = t248 * t1021 * t10858;
    let t10863 = t1004 * t3047;
    let t10866 = t3117 * t3053;
    let t10868 = t676 * t1043;
    let t10870 = t248 * t10868 * t884;
    (t10857, t10860, t10863, t10866, t10870)
}
