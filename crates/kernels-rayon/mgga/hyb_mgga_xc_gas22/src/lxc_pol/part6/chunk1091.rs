//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1091/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1091(t10641: f64, t2188: f64, t10528: f64, t10552: f64, t10557: f64, t10559: f64, t10561: f64, t10563: f64, t10565: f64, t10619: f64, t10621: f64, t10622: f64, t10626: f64, t10631: f64, t10635: f64, t10637: f64, t10640: f64, t260: f64, t3430: f64, t3445: f64, t856: f64) -> (f64, f64) {
    let t10643 = 4.0_f64 * t2188 * t10641;
    let t10644 = -0.34631718211362927517e2_f64 * t3430 * t3445 - 0.35089341735807877242e1_f64 * t856 * t10528 + 0.19751673498613801407e-1_f64 * t260 * t10552 + t10557 + t10559 + t10561 - t10563 + t10565 + t10619 + t10621 - 0.34631718211362927518e2_f64 * t856 * t10622 - 0.17315859105681463759e2_f64 * t856 * t10626 - 0.10254018858216406658e4_f64 * t856 * t10631 - t10635 + t10637 + t10640 - t10643;
    (t10643, t10644)
}
