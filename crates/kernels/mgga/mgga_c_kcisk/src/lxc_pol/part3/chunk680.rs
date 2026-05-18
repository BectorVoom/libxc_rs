//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 680/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk680<F: Float>(t10569: F, t10570: F, t10572: F, t10574: F, t10576: F, t10579: F, t10582: F, t10587: F, t10590: F, t10595: F, t10598: F, t587: F) -> F {
    let t10600 = -t10569 - F::new(0.23744444444444444444e-1) * t10570 + F::new(0.11872222222222222222e-1) * t10572 - F::new(0.35616666666666666666e-1) * t10574 + F::new(0.17808333333333333333e-1) * t10576 - F::new(0.19787037037037037037e-1) * t10579 + F::new(0.71233333333333333332e-1) * t10582 - F::new(0.35616666666666666666e-1) * t10587 - F::new(0.10685e0) * t10590 + F::new(0.10685e0) * t10595 - F::new(0.17808333333333333333e-1) * t10598;
    let t10602 = F::new(0.62182e-1) * t10600 * t587;
    t10602
}
