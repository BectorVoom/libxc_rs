//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1345/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1345<F: Float>(t10532: F, t10533: F, t34321: F, t204: F, t587: F, t2487: F, t6711: F, t6607: F, t8411: F, t34411: F, t6710: F, t10396: F, t21133: F) -> (F, F, F, F, F, F) {
    let t34973 = F::new(0.55213813373645879534e2) * t10532 * t10533 * t34321;
    let t34976 = F::new(0.18404604457881959845e2) * t587 * t204 * t34321;
    let t34979 = F::new(0.87421871174939309262e2) * t2487 * t6711 * t34321;
    let t34983 = F::new(0.21450293971110256002e1) * t8411 * t6607;
    let t34986 = F::new(0.11502877786176224903e2) * t6710 * t6711 * t34411;
    let t34991 = F::new(0.1853729108614466568e0) * t21133 * t10396;
    (t34973, t34976, t34979, t34983, t34986, t34991)
}
