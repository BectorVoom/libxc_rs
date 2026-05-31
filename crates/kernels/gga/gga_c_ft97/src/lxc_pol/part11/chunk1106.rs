//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1106/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1106<F: Float>(t272: F, t41670: F, t41622: F, t2697: F, t41627: F, t41744: F, t41700: F, t41705: F, t41707: F, t41709: F, t41713: F, t41720: F, t41728: F, t41733: F, t41735: F, t41739: F, t43204: F) -> (F, F, F) {
    let t43207 = F::cast_from(1.0_f64) / t272 / t41670;
    let t43208 = t43207 * t41622;
    let t43210 = t2697 * t41627;
    let t43212 = F::cast_from(0.14978012345679012345e1_f64) * t41744;
    let t43223 = F::cast_from(0.234754e0_f64) * t43204 - F::cast_from(0.44016375e0_f64) * t43208 - F::cast_from(0.352131e0_f64) * t43210 + t43212 + F::cast_from(0.86658499999999999998e0_f64) * t41700 + F::cast_from(0.59912049382716049381e0_f64) * t41705 - F::cast_from(0.38514888888888888888e0_f64) * t41707 - F::cast_from(0.25676592592592592592e0_f64) * t41709 + F::cast_from(0.19257444444444444444e1_f64) * t41713 - F::cast_from(0.34663399999999999999e1_f64) * t41720 - F::cast_from(0.28886166666666666666e0_f64) * t41728 + F::cast_from(0.77029777777777777776e0_f64) * t41733 - F::cast_from(0.77029777777777777776e0_f64) * t41735 + F::cast_from(0.11554466666666666666e1_f64) * t41739;
    (t43208, t43210, t43223)
}
