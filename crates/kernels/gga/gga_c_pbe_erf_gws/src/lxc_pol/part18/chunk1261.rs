//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1261/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1261<F: Float>(t54491: F, t898: F, t911: F, t3973: F, t13953: F, t14787: F, t14781: F, t14001: F, t3062: F, t14772: F, t1161: F, t353: F, t51084: F, t859: F) -> (F, F, F, F, F, F, F) {
    let t54492 = F::new(7.0) / F::new(2304.0) * t54491;
    let t54498 = t911 * t898;
    let t54499 = t3973 * t54498;
    let t54504 = t13953 * t14787;
    let t54505 = F::new(7.0) / F::new(144.0) * t54504;
    let t54531 = t13953 * t14781;
    let t54532 = F::new(7.0) / F::new(144.0) * t54531;
    let t54535 = t14001 * t3062;
    let t54536 = F::new(7.0) / F::new(72.0) * t54535;
    let t54537 = t14001 * t14772;
    let t54538 = F::new(7.0) / F::new(72.0) * t54537;
    let t54545 = t859 * t353 * t51084 * t1161;
    (t54492, t54499, t54505, t54532, t54536, t54538, t54545)
}
