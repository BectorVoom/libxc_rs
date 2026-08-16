//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2143/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2143(t1516: f64, t81766: f64, t23127: f64, t4261: f64, t13347: f64, t6621: f64, t131: f64, t6598: f64, t9537: f64, t225: f64, t2627: f64, t236: f64, t25093: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87222 = t81766 * t1516;
    let t87224 = t23127 * t4261;
    let t87226 = t6621 * t13347;
    let t87229 = t6598 * t131 * t9537;
    let t87230 = t225 * t2627;
    let t87233 = t87229 * t87230 * t236 * t25093;
    (t87222, t87224, t87226, t87229, t87230, t87233)
}
