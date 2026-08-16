//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2062/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2062(t23384: f64, t25811: f64, t25407: f64, t25513: f64, t82431: f64, t25726: f64, t25608: f64, t6743: f64, t23631: f64, t61066: f64, t974: f64, t23665: f64, t25524: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t88937 = 0.18277045187202515961e-2_f64 * t23384 * t25811;
    let t88954 = 0.54831135561607547884e-2_f64 * t23384 * t25407;
    let t88992 = 0.36554090374405031922e-2_f64 * t82431 * t25513;
    let t88998 = 0.18277045187202515961e-2_f64 * t82431 * t25726;
    let t89002 = t25608 * t6743;
    let t89033 = t23631 * t974 * t61066;
    let t89049 = 0.54831135561607547884e-2_f64 * t23665 * t25524;
    (t88937, t88954, t88992, t88998, t89002, t89033, t89049)
}
