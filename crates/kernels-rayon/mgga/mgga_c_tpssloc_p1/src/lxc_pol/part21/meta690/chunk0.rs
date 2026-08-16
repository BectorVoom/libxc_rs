//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2505/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2505(t12886: f64, t706: f64, t157: f64, t41284: f64, t12923: f64, t12939: f64, t2244: f64, t2250: f64, t4194: f64, t46528: f64, t816: f64, t4159: f64, t9541: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47172 = t706 * t12886;
    let t47176 = t41284 * t157;
    let t47180 = t12939 * t12923 * t2244;
    let t47185 = t4194 * t12923 * t2250;
    let t47220 = t46528 * t816;
    let t47230 = t9541 * t4159;
    (t47172, t47176, t47180, t47185, t47220, t47230)
}
