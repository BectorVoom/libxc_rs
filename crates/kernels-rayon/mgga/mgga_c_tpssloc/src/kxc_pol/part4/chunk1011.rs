//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1011/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1011(t16816: f64, t16839: f64, t4180: f64, t4182: f64, t5593: f64, t9638: f64, t5527: f64, t776: f64, t820: f64, t9607: f64, t16753: f64, t819: f64) -> (f64, f64, f64, f64, f64) {
    let t16841 = t4180 * t16839 * t16816;
    let t16845 = t4180 * t16839 * t4182;
    let t16848 = t9638 * t5593;
    let t16851 = t5527 * t776;
    let t16853 = t9607 * t820 * t16851;
    let t16859 = t819 * t820 * t16753;
    (t16841, t16845, t16848, t16853, t16859)
}
