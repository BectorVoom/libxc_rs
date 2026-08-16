//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1162/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1162<F: Float>(t6614: F, t831: F, t1899: F, t838: F, t234: F, t59: F, t240: F) -> (F, F, F, F) {
    let t6615 = t6614 * t831;
    let t6617 = t1899 * t838;
    let t6618 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t6617;
    let t6619 = t234 * t59;
    let t6620 = t6619 * t240;
    (t6615, t6618, t6619, t6620)
}
