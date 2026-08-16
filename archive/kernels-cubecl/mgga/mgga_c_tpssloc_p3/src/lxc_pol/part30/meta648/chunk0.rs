//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2062/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2062<F: Float>(t23384: F, t25811: F, t25407: F, t25513: F, t82431: F, t25726: F, t25608: F, t6743: F, t23631: F, t61066: F, t974: F, t23665: F, t25524: F) -> (F, F, F, F, F, F, F) {
    let t88937 = F::cast_from(0.18277045187202515961e-2_f64) * t23384 * t25811;
    let t88954 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25407;
    let t88992 = F::cast_from(0.36554090374405031922e-2_f64) * t82431 * t25513;
    let t88998 = F::cast_from(0.18277045187202515961e-2_f64) * t82431 * t25726;
    let t89002 = t25608 * t6743;
    let t89033 = t23631 * t974 * t61066;
    let t89049 = F::cast_from(0.54831135561607547884e-2_f64) * t23665 * t25524;
    (t88937, t88954, t88992, t88998, t89002, t89033, t89049)
}
