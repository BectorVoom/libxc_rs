//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 702/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk702(t2941: f64, t8313: f64, t2902: f64, t4538: f64, t2942: f64, t2894: f64, t426: f64, t1560: f64, t173: f64, t1559: f64, t1476: f64, t8292: f64, t8298: f64, t8301: f64, t8304: f64, t8306: f64, t8311: f64) -> (f64, f64) {
    let t8314 = t2941 * t8313;
    let t8316 = t2902 * t4538;
    let t8317 = t8316 * t2942;
    let t8319 = t426 * t2894;
    let t8321 = t1560 * t173;
    let t8322 = t1559 * t8321;
    let t8324 = t1476 * t2942;
    let t8326 = -0.29524791194193262952e-5_f64 * t8292 - 0.29524791194193262952e-5_f64 * t8298 - 0.43449121406768801912e-4_f64 * t8301 + 0.21724560703384400956e-4_f64 * t8304 + 0.10427789137624512459e-2_f64 * t8306 + 0.60736713313768998074e-4_f64 * t8311 + 0.43449121406768801912e-5_f64 * t8314 + 0.43449121406768801912e-5_f64 * t8317 + 0.20855578275249024918e-2_f64 * t8319 - 0.41201353525991962561e-5_f64 * t8322 - 0.30414384984738161339e-4_f64 * t8324;
    (t8316, t8326)
}
