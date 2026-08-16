//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 779/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk779(t107: f64, t39048: f64, t787: f64, t12251: f64, t1980: f64, t12161: f64, t296: f64, t1: f64, t2021: f64, t2610: f64, t38912: f64, t1381: f64, t3699: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39050 = t787 * t39048 * t107;
    let t39118 = t1980 * t12251;
    let t39121 = t296 * t12161;
    let t39123 = t787 * t39121 * t1;
    let t39145 = t39048 * t1;
    let t39146 = t2021 * t39145;
    let t39149 = t2610 * t38912;
    let t39337 = t3699 * t1381;
    (t39050, t39118, t39123, t39145, t39146, t39149, t39337)
}
