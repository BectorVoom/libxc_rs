//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 923/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk923<F: Float>(t1771: F, t5114: F, t5118: F, t41955: F, t4918: F, t89: F, t5157: F, t8232: F, t5079: F, t5075: F, t5083: F, t5161: F) -> (F, F, F, F, F, F, F, F, F) {
    let t67329 = t1771 * t5114;
    let t67331 = t1771 * t5118;
    let t67420 = t89 * t41955 * t4918;
    let t67421 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t67420;
    let t67746 = t8232 * t5157;
    let t67881 = t8232 * t5079;
    let t67961 = t8232 * t5075;
    let t68001 = t8232 * t5083;
    let t68074 = t8232 * t5161;
    (t67329, t67331, t67420, t67421, t67746, t67881, t67961, t68001, t68074)
}
