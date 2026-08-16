//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1968/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1968<F: Float>(t4147: F, t7311: F, t1925: F, t36: F, t1353: F, t2033: F, t1518: F, t1931: F, t7933: F, t1469: F, t1450: F, t11239: F, t3268: F) -> (F, F, F, F, F, F, F) {
    let t32113 = t4147 * t7311;
    let t32591 = t1925 * t36;
    let t32737 = t2033 * t1353;
    let t33602 = t1931 * t1518;
    let t33651 = t4147 * t7933;
    let t34176 = t32591 * t1469;
    let t35669 = t7933 * t1450;
    let t36870 = t11239 * t3268;
    (t32113, t32737, t33602, t33651, t34176, t35669, t36870)
}
