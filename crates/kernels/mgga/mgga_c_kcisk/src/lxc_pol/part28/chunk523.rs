//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 523/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk523<F: Float>(t10: F, t1797: F, t1701: F, t1705: F, t1704: F, t617: F, t608: F, t606: F, t609: F, t4834: F, t353: F, t579: F, t964: F, t163: F, t657: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4840 = t10 * t1797;
    let t4853 = t1701 * t1705;
    let t4856 = t1704 * t617;
    let t4857 = 1.0 / t4856;
    let t4858 = t608 * t4857;
    let t4864 = 1.0 / t609 / t606;
    let t4868 = 4.0 / 9.0 * t4834;
    let t4876 = 0.39862222222222222223e0 * t4834;
    let t4881 = 1.0/f64::sqrt(t606);
    let t4887 = t353 * t964 * t579;
    let t4888 = 0.27385555555555555555e0 * t4887;
    let t4889 = t163 * t657;
    (t4840, t4853, t4857, t4858, t4864, t4868, t4876, t4881, t4887, t4888, t4889)
}
