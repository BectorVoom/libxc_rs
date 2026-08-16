//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 863/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk863<F: Float>(t15670: F, t366: F, t372: F, t4823: F, t1062: F, t4857: F, t11986: F, t1592: F, t247: F, t1063: F, t11262: F, t1670: F) -> (F, F, F, F, F, F) {
    let t15671 = t15670 * t366;
    let t15696 = t372 * t4823;
    let t15707 = t4857 * t1062;
    let t15711 = t247 * t11986 * t1592;
    let t15712 = t1063 * t15711;
    let t15731 = t11262 * t1670;
    (t15671, t15696, t15707, t15711, t15712, t15731)
}
