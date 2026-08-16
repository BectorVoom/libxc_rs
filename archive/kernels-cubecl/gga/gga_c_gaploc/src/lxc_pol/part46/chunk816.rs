//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 816/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk816<F: Float>(t12859: F, t4342: F, t3207: F, t8042: F, t1016: F, t29096: F, t10405: F, t2482: F, t9267: F, t3338: F, t4130: F, t9272: F) -> (F, F, F, F, F) {
    let t41583 = t4342 * t12859;
    let t41584 = F::cast_from(4.0_f64) * t41583;
    let t41585 = t8042 * t3207;
    let t41586 = t29096 * t1016;
    let t41588 = t9267 * t10405 * t2482;
    let t41590 = t4130 * t3338;
    let t41592 = t9272 * t41590 * t2482;
    (t41584, t41585, t41586, t41588, t41592)
}
