//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2307/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2307(t16596: f64, t16662: f64, t17116: f64, t1877: f64, t2522: f64, t39483: f64, t40732: f64, t4310: f64, t46237: f64, t67146: f64, t67147: f64, t67153: f64, t67154: f64, t67158: f64, t67159: f64, t868: f64) -> f64 {
    let t67160 = -9.0_f64 * t16596 * t17116 * t2522 + 9.0_f64 * t16662 * t2522 * t4310 - t1877 * t67154 * t868 + t39483 - t40732 + t46237 - t67146 + t67147 + t67153 + t67158 + t67159;
    t67160
}
