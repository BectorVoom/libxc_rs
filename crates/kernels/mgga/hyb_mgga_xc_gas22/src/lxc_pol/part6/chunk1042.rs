//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1042/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1042<F: Float>(t3357: F, t8906: F, t4114: F, t809: F, t2234: F, t1347: F, t3352: F, t2188: F, t10528: F, t10552: F, t10557: F, t10559: F, t10561: F, t10563: F, t10565: F, t10619: F, t10621: F, t10622: F, t10626: F, t10631: F, t10635: F, t260: F, t3430: F, t3445: F, t856: F) -> (F, F, F, F, F, F) {
    let t10637 = 0.32163958997385070134e2 * t8906 * t3357;
    let t10638 = t4114 * t809;
    let t10640 = 6.0 * t2234 * t10638;
    let t10641 = t1347 * t3352;
    let t10643 = 4.0 * t2188 * t10641;
    let t10644 = -0.34631718211362927517e2 * t3430 * t3445 - 0.35089341735807877242e1 * t856 * t10528 + 0.19751673498613801407e-1 * t260 * t10552 + t10557 + t10559 + t10561 - t10563 + t10565 + t10619 + t10621 - 0.34631718211362927518e2 * t856 * t10622 - 0.17315859105681463759e2 * t856 * t10626 - 0.10254018858216406658e4 * t856 * t10631 - t10635 + t10637 + t10640 - t10643;
    (t10637, t10638, t10640, t10641, t10643, t10644)
}
