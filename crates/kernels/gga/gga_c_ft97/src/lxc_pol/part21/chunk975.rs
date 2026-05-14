//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 975/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk975<F: Float>(t1053: F, t26590: F, t144: F, t30302: F, t30297: F, t167: F, t30105: F, t574: F, t23997: F, t4724: F, t27028: F, t27051: F, t27110: F, t30176: F, t30180: F, t30184: F, t30189: F, t30193: F, t30197: F, t30201: F, t30205: F, t30209: F, t30214: F, t30221: F, t30225: F, t30229: F) -> (F, F, F, F, F, F, F, F) {
    let t30306 = t26590 * t1053;
    let t30311 = t144 * t30302;
    let t30314 = t144 * t30306;
    let t30317 = t144 * t30297;
    let t30321 = t574 * t167 * t30105;
    let t30324 = t23997 * t4724;
    let t30325 = t144 * t30324;
    let t30343 = t30176 / 3.0 + t30180 / 6.0 + t30184 / 9.0 - 3.0 / 8.0 * t30189 - t30193 / 2.0 + t30197 / 3.0 + 2.0 / 9.0 * t30201 + 2.0 / 3.0 * t30205 - t30209 / 3.0 - t30214 + t27028 / 3.0 - 2.0 / 9.0 * t27051 - 2.0 / 3.0 * t27110 - t30221 / 6.0 - 2.0 / 3.0 * t30225 - 2.0 / 3.0 * t30229;
    (t30306, t30311, t30314, t30317, t30321, t30324, t30325, t30343)
}
