//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1087/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1087<F: Float>(t26461: F, t6976: F, t1992: F, t16040: F, t550: F, t1336: F, t1814: F, t22718: F, t22726: F, t22728: F, t22730: F, t22745: F, t22752: F, t22895: F, t26434: F, t26437: F, t26442: F, t26449: F, t26453: F, t26456: F, t26459: F, t3777: F, t5234: F, t5334: F, t6988: F, t6990: F, t7745: F) -> (F, F, F) {
    let t26462 = t6976 * t26461;
    let t26463 = t1992 * t26462;
    let t26466 = t16040 * t550;
    let t26467 = t6976 * t26466;
    let t26468 = t1992 * t26467;
    let t26470 = F::cast_from(0.82246703342411321825e-2_f64) * t26434 - F::cast_from(0.41123351671205660912e-2_f64) * t26437 + t22718 + t22726 - F::cast_from(0.41123351671205660912e-2_f64) * t22728 - F::cast_from(0.19190897446562641759e-1_f64) * t22730 + t1814 * t6990 - t1336 * t26442 - t5234 * t6988 - t3777 * t7745 + F::cast_from(0.49348022005446793095e-1_f64) * t26449 + F::cast_from(0.19190897446562641759e-1_f64) * t22745 + F::cast_from(0.38381794893125283518e-1_f64) * t22752 + F::cast_from(2.0_f64) * t5334 * t26453 - t1336 * t26456 - t1336 * t26459 - F::cast_from(0.82246703342411321825e-2_f64) * t26463 + F::cast_from(0.82246703342411321824e-2_f64) * t22895 - F::cast_from(0.82246703342411321825e-2_f64) * t26468;
    (t26463, t26468, t26470)
}
