//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 908/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk908<F: Float>(t2085: F, t6414: F, t6387: F, t225: F, t29290: F, t29293: F, t29287: F, t23030: F, t30660: F, t240: F, t241: F, t2627: F, t812: F) -> (F, F, F, F, F, F, F) {
    let t102587 = t2085 * t6414;
    let t102801 = t2085 * t6387;
    let t102917 = t29290 * t225;
    let t102922 = t29293 * t225;
    let t102948 = t29287 * t225;
    let t112676 = F::cast_from(0.52089578783527170489e-1_f64) * t23030 * t30660;
    let t112792 = t812 * t2627 * t240 * t241;
    (t102587, t102801, t102917, t102922, t102948, t112676, t112792)
}
