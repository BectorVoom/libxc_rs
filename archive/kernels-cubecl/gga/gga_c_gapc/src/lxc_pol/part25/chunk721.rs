//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 721/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk721<F: Float>(t2941: F, t8313: F, t2902: F, t4538: F, t2942: F, t2894: F, t426: F, t1560: F, t173: F, t1559: F, t1476: F, t8292: F, t8298: F, t8301: F, t8304: F, t8306: F, t8311: F) -> (F, F, F, F, F, F, F) {
    let t8314 = t2941 * t8313;
    let t8316 = t2902 * t4538;
    let t8317 = t8316 * t2942;
    let t8319 = t426 * t2894;
    let t8321 = t1560 * t173;
    let t8322 = t1559 * t8321;
    let t8324 = t1476 * t2942;
    let t8326 = -F::cast_from(0.29524791194193262952e-5_f64) * t8292 - F::cast_from(0.29524791194193262952e-5_f64) * t8298 - F::cast_from(0.43449121406768801912e-4_f64) * t8301 + F::cast_from(0.21724560703384400956e-4_f64) * t8304 + F::cast_from(0.10427789137624512459e-2_f64) * t8306 + F::cast_from(0.60736713313768998074e-4_f64) * t8311 + F::cast_from(0.43449121406768801912e-5_f64) * t8314 + F::cast_from(0.43449121406768801912e-5_f64) * t8317 + F::cast_from(0.20855578275249024918e-2_f64) * t8319 - F::cast_from(0.41201353525991962561e-5_f64) * t8322 - F::cast_from(0.30414384984738161339e-4_f64) * t8324;
    (t8314, t8316, t8317, t8319, t8322, t8324, t8326)
}
