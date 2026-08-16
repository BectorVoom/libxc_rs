//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 921/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk921(t40182: f64, t40184: f64, t40187: f64, t12865: f64, t1580: f64, t31828: f64, t874: f64, t1445: f64, t597: f64, t10151: f64, t2293: f64, t10557: f64, t9324: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41852 = 0.25561950635947166451e0_f64 * t40182;
    let t41853 = 0.89376224879626066674e-1_f64 * t40184;
    let t41854 = 0.17875244975925213335e0_f64 * t40187;
    let t41863 = t1580 * t12865;
    let t41865 = t31828 * t874;
    let t41867 = t597 * t1445 * t41865;
    let t41869 = t10151 * t2293;
    let t41871 = t597 * t1445 * t41869;
    let t41874 = 0.85801175884441024006e1_f64 * t10557 * t9324;
    (t41852, t41853, t41854, t41863, t41865, t41867, t41869, t41871, t41874)
}
