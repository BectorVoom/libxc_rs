//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 676/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk676<F: Float>(t1476: F, t2942: F, t8292: F, t8298: F, t8301: F, t8304: F, t8306: F, t8311: F, t8314: F, t8317: F, t8319: F, t8322: F, t126: F, t1554: F, t120: F, t1134: F, t991: F) -> (F, F, F, F) {
    let t8324 = t1476 * t2942;
    let t8326 = -0.29524791194193262952e-5 * t8292 - 0.29524791194193262952e-5 * t8298 - 0.43449121406768801912e-4 * t8301 + 0.21724560703384400956e-4 * t8304 + 0.10427789137624512459e-2 * t8306 + 0.60736713313768998074e-4 * t8311 + 0.43449121406768801912e-5 * t8314 + 0.43449121406768801912e-5 * t8317 + 0.20855578275249024918e-2 * t8319 - 0.41201353525991962561e-5 * t8322 - 0.30414384984738161339e-4 * t8324;
    let t8327 = t126 * t1554;
    let t8328 = t120 * t8327;
    let t8330 = t1134 * t991;
    (t8324, t8326, t8328, t8330)
}
