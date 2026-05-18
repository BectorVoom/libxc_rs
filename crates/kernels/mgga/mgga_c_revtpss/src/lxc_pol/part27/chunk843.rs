//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 843/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk843<F: Float>(t1445: F, t3895: F, t2439: F, t1420: F, t2453: F, t3908: F, t4067: F, t786: F, t1364: F, t213: F, t4066: F, t1426: F) -> (F, F, F, F, F) {
    let t10162 = t3895 * t1445;
    let t10163 = t2439 * t10162;
    let t10165 = t2453 * t1420;
    let t10166 = t10165 * t3908;
    let t10168 = t786 * t4067;
    let t10169 = t10168 * t1364;
    let t10171 = t213 * t4066;
    let t10174 = t1420 * t1426;
    (t10163, t10166, t10169, t10171, t10174)
}
