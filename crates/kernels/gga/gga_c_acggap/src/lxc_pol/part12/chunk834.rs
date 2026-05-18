//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 834/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk834<F: Float>(t8973: F, t8981: F, t7840: F, t7845: F, t7848: F, t7865: F, t8291: F, t8292: F, t8294: F, t8963: F, t8967: F, t8971: F, t8975: F, t8979: F, t8983: F) -> F {
    let t9356 = F::new(0.64311027177104605458e-2) * t8973;
    let t9359 = F::new(0.94344276868812456204e-2) * t8981;
    let t9363 = F::new(0.62896184579208304138e-3) * t8963 - F::new(0.94344276868812456207e-3) * t8967 + F::new(0.31448092289604152069e-3) * t8971 + t9356 - F::new(0.56606566121287473724e-2) * t8975 - F::new(0.42874018118069736972e-3) * t8979 - t9359 + F::new(0.25724410870841842183e-2) * t8983 + F::new(0.31448092289604152069e-3) * t7840 + F::new(0.20965394859736101379e-3) * t7845 - t7848 + t8291 + t8292 + t8294 - t7865;
    t9363
}
