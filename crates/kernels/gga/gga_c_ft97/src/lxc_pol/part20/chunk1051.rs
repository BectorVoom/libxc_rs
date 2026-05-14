//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1051/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1051<F: Float>(t263: F, t27742: F, t1425: F, t9568: F, t9570: F, t6837: F, t7514: F, t1403: F, t27952: F, t681: F, t24178: F, t6745: F, t10157: F, t13863: F, t14053: F, t14075: F, t14358: F, t1454: F, t193: F, t2354: F, t2405: F, t2409: F, t2413: F, t24182: F, t24204: F, t24231: F, t27993: F, t28036: F, t28037: F, t6002: F, t6003: F, t6752: F, t684: F, t96796: F, t98195: F) -> (F, F) {
    let t107910 = t27742 * t263;
    let t107919 = t9568 * t1425;
    let t107920 = t263 * t9570;
    let t107937 = t7514 * t6837;
    let t107943 = 2.0 / 9.0 * t1403 * t681 * t27952;
    let t107945 = 2.0 / 9.0 * t6745 * t24178;
    let t107949 = 2.0 * t6002 * t10157 * t6003 * t14053 - t24204 * t27993 / 9.0 - t6002 * t2354 * t107910 * t684 / 9.0 - t6002 * t28036 * t28037 * t14075 / 27.0 - 5.0 / 81.0 * t6002 * t107919 * t107920 * t13863 + t6002 * t24231 * t6752 * t2413 / 9.0 + 2.0 / 27.0 * t6002 * t98195 * t6752 * t2405 - 2.0 / 9.0 * t6002 * t24231 * t6752 * t2409 + t1403 * t193 * t107937 * t24182 + t107943 + t107945 - 2.0 * t14358 * t1454 - t96796 / 3.0;
    (t107937, t107949)
}
