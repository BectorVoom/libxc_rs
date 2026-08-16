//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 947/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk947(t1060: f64, t25499: f64, t4688: f64, t6800: f64, t6799: f64, t23665: f64, t7611: f64, t1936: f64, t362: f64, t2775: f64, t381: f64, t3961: f64) -> (f64, f64, f64, f64, f64) {
    let t25500 = t25499 * t1060;
    let t25502 = t4688 * t6800;
    let t25503 = t6799 * t25502;
    let t25508 = t23665 * t7611;
    let t25510 = t1936 * t362;
    let t25511 = t381 * t2775;
    let t25512 = t25511 * t3961;
    (t25500, t25503, t25508, t25510, t25512)
}
