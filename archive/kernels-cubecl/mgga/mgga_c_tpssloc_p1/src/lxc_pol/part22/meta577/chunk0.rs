//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2086/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2086<F: Float>(t1184: F, t15394: F, t11147: F, t460: F, t15418: F, t11545: F, t135: F, t3439: F, t698: F, t1176: F, t697: F, t11153: F) -> (F, F, F, F, F, F, F, F) {
    let t44504 = t15394 * t1184;
    let t44505 = t460 * t11147;
    let t44525 = t15418 * t1184;
    let t44562 = t135 * t11545;
    let t44571 = t698 * t3439;
    let t44583 = t697 * t1176;
    let t44584 = t44583 * t1184;
    let t44607 = t460 * t11153;
    (t44504, t44505, t44525, t44562, t44571, t44583, t44584, t44607)
}
