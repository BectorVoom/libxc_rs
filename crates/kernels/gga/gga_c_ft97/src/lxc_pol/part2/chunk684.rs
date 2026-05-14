//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 684/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk684<F: Float>(t11484: F, t11546: F, t11591: F, t11822: F, t11876: F, t11910: F, t11997: F, t12055: F, t103: F, t11801: F, t108: F, t11420: F, t11424: F, t11427: F, t11538: F, t11542: F, t11816: F, t11838: F, t11961: F, t2976: F, t497: F, t88: F) -> (F,) {
    let t12058 = t11484 + t11546 + t11591 + t11822 + t11876 + t11910 + t11997 + t12055;
    let t12062 = t11801 * t103;
    let t12067 = -t108 * t11420 - 2.0 * t108 * t11424 - t108 * t11427 - t12058 * t88 - 2.0 * t2976 * t497 - 2.0 * t11538 - 4.0 * t11542 - 2.0 * t11816 - 4.0 * t11838 - 2.0 * t11961 + 2.0 * t12062;
    (t12067,)
}
