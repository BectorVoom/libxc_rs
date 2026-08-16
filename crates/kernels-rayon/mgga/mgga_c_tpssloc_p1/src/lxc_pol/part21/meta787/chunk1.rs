//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2739/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2739(t20085: f64, t3914: f64, t39844: f64, t5160: f64, t57215: f64, t57216: f64, t57218: f64, t57219: f64, t57220: f64, t57221: f64, t57222: f64, t57223: f64, t57224: f64, t57225: f64) -> f64 {
    let t57815 = 2.0_f64 * t20085 * t3914 * t5160 + t39844 + t57215 - t57216 + t57218 - t57219 - t57220 + t57221 - t57222 - t57223 - t57224 - t57225;
    t57815
}
