//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1110/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1110<F: Float>(t54504: F, t13953: F, t14781: F, t14001: F, t3062: F, t14772: F, t1161: F, t353: F, t51084: F, t859: F, t4183: F, t4386: F, t810: F, t14466: F, t14765: F, t3074: F, t4395: F) -> (F, F, F, F, F, F, F, F) {
    let t54505 = 7.0 / 144.0 * t54504;
    let t54531 = t13953 * t14781;
    let t54532 = 7.0 / 144.0 * t54531;
    let t54535 = t14001 * t3062;
    let t54536 = 7.0 / 72.0 * t54535;
    let t54537 = t14001 * t14772;
    let t54538 = 7.0 / 72.0 * t54537;
    let t54545 = t859 * t353 * t51084 * t1161;
    let t54550 = t4386 * t353 * t4183 * t810;
    let t54566 = t14001 * t14466;
    let t54567 = 7.0 / 72.0 * t54566;
    let t54580 = t3074 * t4395 * t14765;
    (t54505, t54532, t54536, t54538, t54545, t54550, t54567, t54580)
}
