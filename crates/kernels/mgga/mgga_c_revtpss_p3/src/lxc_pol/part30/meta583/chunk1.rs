//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2038/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2038<F: Float>(t2453: F, t3908: F, t7275: F, t1399: F, t2434: F, t25880: F, t25899: F, t3924: F, t676: F, t2022: F, t9646: F, t9648: F) -> (F, F, F, F, F, F) {
    let t94616 = t2453 * t7275 * t3908;
    let t94633 = t2434 * t1399;
    let t94634 = t25880 * t94633;
    let t94635 = t25899 * t94634;
    let t94639 = t676 * t3924;
    let t94640 = t25880 * t94639;
    let t94641 = t25899 * t94640;
    let t94648 = F::cast_from(0.19637199382202157274e-3_f64) * t9646 * t2022 * t9648;
    (t94616, t94634, t94635, t94640, t94641, t94648)
}
