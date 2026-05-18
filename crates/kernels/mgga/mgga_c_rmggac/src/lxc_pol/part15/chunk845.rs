//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 845/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk845<F: Float>(t41735: F, t22: F, t235: F, t34812: F, t275: F, t8887: F, t1982: F, t2314: F, t35512: F, t2289: F, t7921: F, t6355: F, t7707: F) -> (F, F, F, F, F, F) {
    let t41736 = F::new(0.36366215538993788972e-1) * t41735;
    let t41738 = t235 * t34812 * t22;
    let t41763 = F::new(2.0) * t275 * t8887;
    let t41767 = t2314 * t35512 * t1982;
    let t41774 = t7921 * t2289;
    let t41789 = t6355 * t7707;
    (t41736, t41738, t41763, t41767, t41774, t41789)
}
