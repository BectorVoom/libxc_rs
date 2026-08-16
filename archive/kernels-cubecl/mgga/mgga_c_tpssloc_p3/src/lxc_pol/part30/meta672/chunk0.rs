//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2101/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2101<F: Float>(t87140: F, t87155: F, t87177: F, t87243: F, t87304: F, t87345: F, t87403: F, t87405: F, t87432: F, t87653: F, t87666: F, t87718: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t92513 = F::cast_from(0.3289868133696452873e-1_f64) * t87140;
    let t92516 = F::cast_from(0.52089578783527170489e-1_f64) * t87155;
    let t92543 = F::cast_from(0.16449340668482264365e-1_f64) * t87177;
    let t92597 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t87243;
    let t92633 = F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t87304;
    let t92652 = F::cast_from(119.0_f64) / F::cast_from(864.0_f64) * t87345;
    let t92676 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t87403;
    let t92677 = F::cast_from(0.10541775202358879834e-2_f64) * t87405;
    let t92689 = F::cast_from(0.22608743412718618878e-1_f64) * t87432;
    let t92781 = F::cast_from(0.16449340668482264365e-1_f64) * t87653;
    let t92794 = F::cast_from(0.12793931631041761173e0_f64) * t87666;
    let t92817 = F::cast_from(0.10417915756705434098e0_f64) * t87718;
    (t92513, t92516, t92543, t92597, t92633, t92652, t92676, t92677, t92689, t92781, t92794, t92817)
}
