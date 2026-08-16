//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2085/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2085<F: Float>(t86916: F, t86955: F, t86991: F, t87068: F, t87080: F, t87140: F, t87155: F, t87177: F, t87243: F, t87304: F, t87345: F, t87403: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t92406 = F::cast_from(0.3289868133696452873e-1_f64) * t86916;
    let t92432 = F::cast_from(0.12793931631041761173e0_f64) * t86955;
    let t92458 = F::cast_from(0.12793931631041761173e0_f64) * t86991;
    let t92492 = F::cast_from(0.52089578783527170489e-1_f64) * t87068;
    let t92497 = F::cast_from(0.12793931631041761173e0_f64) * t87080;
    let t92513 = F::cast_from(0.3289868133696452873e-1_f64) * t87140;
    let t92516 = F::cast_from(0.52089578783527170489e-1_f64) * t87155;
    let t92543 = F::cast_from(0.16449340668482264365e-1_f64) * t87177;
    let t92597 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t87243;
    let t92633 = F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t87304;
    let t92652 = F::cast_from(119.0_f64) / F::cast_from(864.0_f64) * t87345;
    let t92676 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t87403;
    (t92406, t92432, t92458, t92492, t92497, t92513, t92516, t92543, t92597, t92633, t92652, t92676)
}
