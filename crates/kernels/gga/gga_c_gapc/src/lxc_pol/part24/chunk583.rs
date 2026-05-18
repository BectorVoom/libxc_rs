//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 583/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk583<F: Float>(t2874: F, t2876: F, t2882: F, t2887: F, t2892: F, t2895: F, t2897: F, t2900: F, t2904: F, t2907: F, t3477: F, t1104: F, t575: F) -> (F, F) {
    let t3478 = -F::new(0.3475929712541504153e-2) * t2874 + F::new(0.20855578275249024918e-2) * t2876 - F::new(0.20855578275249024918e-2) * t2882 - F::new(0.69518594250830083059e-4) * t2887 + F::new(0.12360406057797588768e-3) * t2892 + F::new(0.20855578275249024918e-2) * t2895 + F::new(0.27517776890953574545e-3) * t2897 - F::new(0.20855578275249024918e-2) * t2900 - F::new(0.26319242435966565832e-3) * t2904 + F::new(0.60736713313768998073e-4) * t2907 + t3477;
    let t3480 = t1104 * t575;
    (t3478, t3480)
}
