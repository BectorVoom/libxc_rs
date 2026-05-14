//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1124/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1124<F: Float>(t24220: F, t6745: F, t1403: F, t27964: F, t681: F, t24237: F, t28020: F, t10052: F, t10157: F, t1091: F, t1425: F, t193: F, t2354: F, t24204: F, t24232: F, t24240: F, t24257: F, t27924: F, t27971: F, t27997: F, t3683: F, t3704: F, t3821: F, t3837: F, t41402: F, t6002: F, t6008: F, t6192: F, t6754: F, t766: F, t771: F, t96812: F, t97255: F) -> (F,) {
    let t109577 = t6745 * t24220 / 9.0;
    let t109589 = 2.0 / 9.0 * t1403 * t681 * t27964;
    let t109597 = t24237 * t28020 / 27.0;
    let t109601 = -24.0 * t41402 * t27971 - 24.0 * t10052 * t27924 * t766 + 2.0 * t24204 * t27997 + 2.0 * t6002 * t10157 * t24240 * t3837 - t109577 - t6002 * t2354 * t96812 * t1091 / 18.0 - 2.0 / 3.0 * t1403 * t193 * t6008 * t771 * t3821 + t109589 + t1403 * t3704 * t1425 * t24232 / 9.0 - 2.0 * t3683 * t6192 + t109597 - t97255 / 9.0 - t24257 * t6754 / 3.0;
    (t109601,)
}
