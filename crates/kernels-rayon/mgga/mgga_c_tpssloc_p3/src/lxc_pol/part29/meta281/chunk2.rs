//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1298/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1298(t1458: f64, t7266: f64, t7675: f64, t7678: f64, t7680: f64, t7983: f64, t113: f64, t1442: f64, t1459: f64, t1774: f64, t1849: f64, t2114: f64, t2165: f64, t2167: f64, t510: f64, t574: f64, t652: f64, t7457: f64, t7460: f64, t7463: f64, t7470: f64, t7686: f64, t7690: f64, t7755: f64, t7757: f64, t7989: f64, t8103: f64) -> (f64, f64) {
    let t8107 = 2.0_f64 * t1458 * t7266 + t7675 + t7678 + t7680 + t7983;
    let t8110 = -t113 * t8103 - t1442 * t2165 - 2.0_f64 * t1459 * t7266 - t1774 * t2114 + t1849 * t2167 - t510 * t7983 + t574 * t8107 - 2.0_f64 * t652 * t7989 - t7457 - t7460 - t7463 - t7470 + t7686 + t7690 + t7755 - t7757;
    (t8107, t8110)
}
