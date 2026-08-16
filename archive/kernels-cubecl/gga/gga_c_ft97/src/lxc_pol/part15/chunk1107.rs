//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1107/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1107<F: Float>(t2594: F, t446: F, t88184: F, t4965: F, t5053: F, t9744: F, t66832: F, t80677: F, t80679: F, t88143: F, t88147: F, t88151: F, t88155: F, t88159: F, t88163: F, t88167: F, t88171: F, t88178: F, t88182: F) -> (F, F, F, F) {
    let t88186 = t446 * t2594 * t88184;
    let t88188 = t4965 * t5053;
    let t88190 = t446 * t9744 * t88188;
    let t88192 = F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t88143 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t88147 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t88151 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t88155 - F::cast_from(4.0_f64) * t88159 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t88163 + F::cast_from(8.0_f64) * t88167 + F::cast_from(8.0_f64) * t88171 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t66832 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t80677 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t80679 + F::cast_from(8.0_f64) * t88178 + F::cast_from(2.0_f64) * t88182 + F::cast_from(8.0_f64) * t88186 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t88190;
    (t88186, t88188, t88190, t88192)
}
