//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1206/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1206<F: Float>(t209: F, t31520: F, t31562: F, t31578: F, t31647: F, t31686: F, t31728: F, t31761: F, t31814: F, t31861: F, t31901: F, t31925: F, t31970: F, t32004: F, t32017: F, t32051: F, t32085: F) -> F {
    let t32090 = (t31520 + t31562 + t31578 + t31647 + t31686 + t31728 + t31761 + t31814 + t31861 + t31901 + t31925 + t31970 + t32004 + t32017 + t32051 + t32085) * t209;
    t32090
}
