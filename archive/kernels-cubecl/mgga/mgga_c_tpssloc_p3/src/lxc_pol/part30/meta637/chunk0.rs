//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2047/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2047<F: Float>(t25425: F, t82431: F, t25816: F, t25443: F, t1049: F, t7577: F, t7557: F, t82573: F, t23384: F, t25785: F, t25447: F, t1625: F, t6733: F) -> (F, F, F, F, F, F, F, F) {
    let t88069 = F::cast_from(0.36554090374405031922e-2_f64) * t82431 * t25425;
    let t88075 = F::cast_from(0.18277045187202515961e-2_f64) * t82431 * t25816;
    let t88083 = F::cast_from(0.18277045187202515961e-2_f64) * t82431 * t25443;
    let t88089 = t7577 * t1049;
    let t88096 = F::cast_from(0.14621636149762012769e-1_f64) * t82573 * t7557;
    let t88100 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25785;
    let t88102 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25447;
    let t88105 = t6733 * t1625;
    (t88069, t88075, t88083, t88089, t88096, t88100, t88102, t88105)
}
