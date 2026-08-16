//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2043/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2043<F: Float>(t12328: F, t1333: F, t1336: F, t2690: F, t3788: F, t67: F, t6924: F, t246: F, t12250: F, t1307: F, t39037: F, t522: F) -> (F, F, F, F, F, F) {
    let t40145 = t1333 * t12328;
    let t40159 = t1336 * t3788 * t2690;
    let t40167 = t6924 * t67;
    let t40168 = t40167 * t246;
    let t40192 = t12250 * t1307;
    let t40224 = F::cast_from(840.0_f64) * t39037 * t522;
    (t40145, t40159, t40167, t40168, t40192, t40224)
}
