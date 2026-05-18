//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 883/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk883<F: Float>(t16377: F, t16454: F, t16559: F, t16607: F, t1396: F, t1468: F, t1464: F, t4137: F, t5752: F, t1928: F, t4134: F, t4136: F) -> (F, F, F, F, F) {
    let t16609 = t16377 + t16454 + t16559 + t16607;
    let t16610 = t1396 * t16609;
    let t16611 = t1468 * t16610;
    let t16612 = t1464 * t16611;
    let t16614 = t5752 * t4137;
    let t16615 = t1464 * t16614;
    let t16617 = t1928 * t4134;
    let t16618 = t16617 * t4136;
    (t16609, t16610, t16612, t16615, t16618)
}
