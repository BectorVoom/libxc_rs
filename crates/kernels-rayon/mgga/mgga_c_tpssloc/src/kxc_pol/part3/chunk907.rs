//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 907/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk907(t10228: f64, t973: f64, t135: f64, t2978: f64, t2981: f64, t4509: f64, t984: f64, t2770: f64, t343: f64, t2244: f64, t2987: f64, t3008: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10229 = t973 * t10228;
    let t10231 = t135 * t2978;
    let t10232 = t10231 * t2981;
    let t10233 = t973 * t10232;
    let t10235 = t4509 * t984;
    let t10236 = t343 * t2770;
    let t10237 = t10236 * t2244;
    let t10241 = t2987 * t3008;
    (t10229, t10231, t10233, t10235, t10236, t10237, t10241)
}
