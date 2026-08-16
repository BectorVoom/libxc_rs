//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1888/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1888(t26461: f64, t6976: f64, t1992: f64, t16040: f64, t550: f64, t1336: f64, t1814: f64, t22718: f64, t22726: f64, t22728: f64, t22730: f64, t22745: f64, t22752: f64, t22895: f64, t26434: f64, t26437: f64, t26442: f64, t26449: f64, t26453: f64, t26456: f64, t26459: f64, t3777: f64, t5234: f64, t5334: f64, t6988: f64, t6990: f64, t7745: f64) -> (f64, f64, f64, f64) {
    let t26462 = t6976 * t26461;
    let t26463 = t1992 * t26462;
    let t26466 = t16040 * t550;
    let t26467 = t6976 * t26466;
    let t26468 = t1992 * t26467;
    let t26470 = 0.82246703342411321825e-2_f64 * t26434 - 0.41123351671205660912e-2_f64 * t26437 + t22718 + t22726 - 0.41123351671205660912e-2_f64 * t22728 - 0.19190897446562641759e-1_f64 * t22730 + t1814 * t6990 - t1336 * t26442 - t5234 * t6988 - t3777 * t7745 + 0.49348022005446793095e-1_f64 * t26449 + 0.19190897446562641759e-1_f64 * t22745 + 0.38381794893125283518e-1_f64 * t22752 + 2.0_f64 * t5334 * t26453 - t1336 * t26456 - t1336 * t26459 - 0.82246703342411321825e-2_f64 * t26463 + 0.82246703342411321824e-2_f64 * t22895 - 0.82246703342411321825e-2_f64 * t26468;
    (t26462, t26466, t26467, t26470)
}
