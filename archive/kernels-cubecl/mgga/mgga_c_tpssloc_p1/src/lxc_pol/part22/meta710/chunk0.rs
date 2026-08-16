//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2306/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2306<F: Float>(t57965: F, t40722: F, t40733: F, t57992: F, t185: F, t67060: F, t707: F, t21066: F, t2752: F, t145: F, t67083: F, t20767: F, t751: F) -> (F, F, F, F, F, F, F, F) {
    let t67137 = F::cast_from(36.0_f64) * t57965;
    let t67141 = F::cast_from(0.56968947174242584612e-3_f64) * t40722;
    let t67146 = F::cast_from(0.35089341735807877242e1_f64) * t40733;
    let t67147 = F::cast_from(12.0_f64) * t57992;
    let t67153 = F::cast_from(4.0_f64) * t707 * t185 * t67060;
    let t67154 = t21066 * t2752;
    let t67158 = t145 * t67083 * t185;
    let t67159 = t20767 * t751;
    (t67137, t67141, t67146, t67147, t67153, t67154, t67158, t67159)
}
