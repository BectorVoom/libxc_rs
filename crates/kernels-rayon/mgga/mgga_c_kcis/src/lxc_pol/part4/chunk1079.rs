//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1079/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1079(t1045: f64, t13677: f64, t331: f64, t4845: f64, t1027: f64, t4849: f64, t10114: f64, t4840: f64, t4852: f64, t1670: f64, t2944: f64, t4625: f64, t934: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13678 = t13677 * t1045;
    let t13682 = 0.93706135855523581992e-2_f64 * t331 * t4845;
    let t13684 = 0.28111840756657074598e-1_f64 * t1027 * t4849;
    let t13686 = t10114 * t4840;
    let t13689 = 0.93706135855523581992e-2_f64 * t1027 * t4852;
    let t13691 = t1670 * t2944;
    let t13696 = t4625 * t934;
    (t13678, t13682, t13684, t13686, t13689, t13691, t13696)
}
