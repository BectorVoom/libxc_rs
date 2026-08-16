//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 974/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk974<F: Float>(t3111: F, t3188: F, t3075: F, t999: F, t247: F, t3116: F, t11173: F, t373: F, t371: F, t372: F, t3211: F, t3215: F) -> (F, F, F, F, F) {
    let t11802 = t3188 * t3111;
    let t11804 = t3075 * t999;
    let t11806 = t247 * t3116 * t11804;
    let t11809 = t373 * t11173;
    let t11811 = t371 * t372 * t11809;
    let t11814 = t3211 * t3215;
    (t11802, t11804, t11806, t11811, t11814)
}
