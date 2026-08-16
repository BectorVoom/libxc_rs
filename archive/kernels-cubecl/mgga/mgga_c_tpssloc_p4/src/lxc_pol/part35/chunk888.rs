//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 888/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk888<F: Float>(t13908: F, t973: F, t10508: F, t1616: F, t248: F, t1020: F, t3069: F, t4669: F, t1612: F, t3082: F, t1606: F, t698: F) -> (F, F, F, F, F) {
    let t13909 = t973 * t13908;
    let t13965 = t248 * t10508 * t1616;
    let t13966 = t1020 * t13965;
    let t13995 = t4669 * t3069;
    let t14117 = t1612 * t3082;
    let t14159 = t698 * t1606;
    (t13909, t13966, t13995, t14117, t14159)
}
