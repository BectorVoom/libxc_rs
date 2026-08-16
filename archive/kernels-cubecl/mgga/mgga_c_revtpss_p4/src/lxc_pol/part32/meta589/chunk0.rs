//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1918/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1918<F: Float>(t102972: F, t25411: F, t15003: F, t95773: F, t1579: F, t26550: F, t103005: F, t25375: F, t26506: F, t27216: F, t786: F, t7998: F, t867: F) -> (F, F, F, F, F, F) {
    let t103023 = F::cast_from(0.25702851531048074406e-1_f64) * t25411 * t102972;
    let t103030 = t95773 * t15003;
    let t103037 = t26550 * t1579;
    let t103047 = F::cast_from(0.28912093960683998208e-1_f64) * t25375 * t103005;
    let t103063 = t27216 * t26506;
    let t103067 = t786 * t7998 * t867;
    (t103023, t103030, t103037, t103047, t103063, t103067)
}
