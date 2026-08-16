//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1417/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1417(t6607: f64, t8411: f64, t34411: f64, t6710: f64, t6711: f64, t10396: f64, t21133: f64, t10140: f64, t4614: f64, t597: f64, t10359: f64, t4953: f64) -> (f64, f64, f64, f64, f64) {
    let t34983 = 0.21450293971110256002e1_f64 * t8411 * t6607;
    let t34986 = 0.11502877786176224903e2_f64 * t6710 * t6711 * t34411;
    let t34991 = 0.1853729108614466568e0_f64 * t21133 * t10396;
    let t34994 = 0.30674340763136599742e2_f64 * t597 * t4614 * t10140;
    let t34996 = 0.18404604457881959845e2_f64 * t4953 * t10359;
    (t34983, t34986, t34991, t34994, t34996)
}
