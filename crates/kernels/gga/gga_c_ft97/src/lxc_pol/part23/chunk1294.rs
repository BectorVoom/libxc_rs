//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1294/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1294<F: Float>(t27915: F, t6745: F, t1403: F, t30910: F, t681: F, t1424: F, t5179: F, t109577: F, t109589: F, t109597: F, t18471: F, t18712: F, t193: F, t2354: F, t24181: F, t24204: F, t24231: F, t24232: F, t24245: F, t27894: F, t28030: F, t28031: F, t28036: F, t28037: F, t30919: F, t30939: F, t4934: F, t4969: F, t6002: F, t684: F, t6840: F, t771: F, t96824: F) -> (F,) {
    let t125112 = t6745 * t27915;
    let t125134 = t1403 * t681 * t30910;
    let t125138 = t1424 * t5179;
    let t125143 = t6002 * t2354 * t24245 * t4969 / 9.0 + t24204 * t30939 / 9.0 - t109577 - t125112 / 9.0 + t6002 * t24231 * t24232 * t18471 / 9.0 + t6002 * t28030 * t28031 * t18712 / 9.0 - t6002 * t28036 * t28037 * t18712 / 27.0 + t109589 + t1403 * t193 * t96824 * t30919 + t1403 * t193 * t24181 * t771 * t4934 + t125134 / 9.0 + t109597 + t27894 * t6840 / 3.0 - t6002 * t2354 * t125138 * t684 / 18.0;
    (t125143,)
}
