//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1327/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1327<F: Float>(t114585: F, t1411: F, t1440: F, t8240: F, t1286: F, t33608: F, t80050: F, t33373: F, t33451: F, t109717: F, t34723: F, t113997: F, t114271: F, t114302: F, t114305: F, t114315: F, t26476: F, t32019: F, t33389: F, t33400: F, t33460: F, t34693: F, t6204: F, t9446: F, t9452: F, t9796: F) -> (F, F, F, F) {
    let t119227 = t1411 * t114585 * t8240 * t1440;
    let t119231 = t1411 * t33608 * t80050 * t1286;
    let t119235 = t33373 * t33451;
    let t119238 = t1411 * t109717 * t34723;
    let t119250 = -t114271 - 0.20833333333333333334e-1 * t32019 * t34693 - 0.1492375e-1 * t119227 + 0.33163888888888888888e-2 * t119231 - 0.55555555555555555557e-1 * t113997 * t9796 + 0.69444444444444444447e-2 * t119235 + 0.88437037037037037035e-2 * t119238 + 0.10416666666666666667e-1 * t9446 * t6204 * t9452 * t26476 - 0.7369753086419753086e-3 * t114302 - 0.80416666666666666668e-2 * t114305 - 0.20833333333333333334e-1 * t33373 * t33400 - 0.24125000000000000001e-1 * t33460 * t33389 - t114315;
    (t119227, t119231, t119238, t119250)
}
