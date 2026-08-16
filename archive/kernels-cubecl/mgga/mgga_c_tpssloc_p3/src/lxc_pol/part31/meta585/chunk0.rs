//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1826/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1826<F: Float>(t215: F, t6916: F, t225: F, t3787: F, t562: F, t22751: F, t26385: F, t81149: F, t81187: F, t81197: F, t26389: F, t26467: F, t6914: F) -> (F, F, F, F, F, F, F, F, F) {
    let t91004 = t6916 * t215;
    let t91005 = t225 * t3787;
    let t91006 = t91005 * t562;
    let t91010 = t22751 * t26385;
    let t91018 = F::cast_from(0.16449340668482264365e-1_f64) * t81149;
    let t91043 = F::cast_from(0.25587863262083522346e0_f64) * t81187;
    let t91045 = F::cast_from(0.3289868133696452873e-1_f64) * t81197;
    let t91064 = t22751 * t26389;
    let t91076 = t6914 * t26467;
    (t91004, t91005, t91006, t91010, t91018, t91043, t91045, t91064, t91076)
}
