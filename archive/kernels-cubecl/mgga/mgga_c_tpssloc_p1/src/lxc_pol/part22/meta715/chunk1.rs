//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2322/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2322<F: Float>(t39529: F, t40779: F, t40784: F, t40790: F, t40793: F, t40797: F, t40799: F, t46311: F, t67191: F, t67204: F, t67206: F, t67207: F, t67210: F, t67211: F, t67212: F, t67214: F, t67215: F) -> F {
    let t67455 = t67191 - t39529 + t67204 + t67206 + t67207 - t67210 + t67211 - t40779 + t67212 + t40784 - t46311 + t67214 + t40790 + t40793 + t67215 + t40797 + t40799;
    t67455
}
