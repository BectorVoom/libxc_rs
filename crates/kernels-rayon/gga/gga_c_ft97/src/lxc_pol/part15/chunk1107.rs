//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1107/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1107(t2594: f64, t446: f64, t88184: f64, t4965: f64, t5053: f64, t9744: f64, t66832: f64, t80677: f64, t80679: f64, t88143: f64, t88147: f64, t88151: f64, t88155: f64, t88159: f64, t88163: f64, t88167: f64, t88171: f64, t88178: f64, t88182: f64) -> (f64, f64, f64, f64) {
    let t88186 = t446 * t2594 * t88184;
    let t88188 = t4965 * t5053;
    let t88190 = t446 * t9744 * t88188;
    let t88192 = 40.0_f64 / 27.0_f64 * t88143 + 4.0_f64 / 3.0_f64 * t88147 + 8.0_f64 / 3.0_f64 * t88151 - 8.0_f64 / 9.0_f64 * t88155 - 4.0_f64 * t88159 - 16.0_f64 / 3.0_f64 * t88163 + 8.0_f64 * t88167 + 8.0_f64 * t88171 + 16.0_f64 / 9.0_f64 * t66832 - 8.0_f64 / 3.0_f64 * t80677 + 8.0_f64 / 3.0_f64 * t80679 + 8.0_f64 * t88178 + 2.0_f64 * t88182 + 8.0_f64 * t88186 + 4.0_f64 / 3.0_f64 * t88190;
    (t88186, t88188, t88190, t88192)
}
