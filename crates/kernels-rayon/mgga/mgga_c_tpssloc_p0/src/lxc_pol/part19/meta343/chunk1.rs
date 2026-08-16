//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1229/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1229(t172: f64, t763: f64, t9915: f64, t184: f64, t4194: f64, t607: f64, t9258: f64, t12939: f64, t2244: f64, t9681: f64, t2371: f64, t9716: f64) -> (f64, f64, f64, f64) {
    let t41265 = t9915 * t172 * t763;
    let t41266 = 0.23392894490538584828e1_f64 * t41265;
    let t41270 = 48.0_f64 * t4194 * t184 * t9258 * t607;
    let t41273 = 144.0_f64 * t12939 * t9681 * t2244;
    let t41274 = t9716 * t2371;
    (t41266, t41270, t41273, t41274)
}
