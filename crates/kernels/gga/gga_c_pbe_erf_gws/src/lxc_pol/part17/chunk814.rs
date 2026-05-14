//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 814/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk814<F: Float>(t1017: F, t1764: F, t1403: F, t1827: F, t587: F, t1000: F, t1406: F, t1821: F, t1820: F, t197: F, t2620: F, t7355: F, t1660: F, t331: F, t7346: F, t1802: F, t1885: F) -> (F, F, F, F, F, F) {
    let t7685 = t1017 * t1764;
    let t7686 = t7685 * t1403;
    let t7687 = t1827 * t7686;
    let t7689 = 8.0 / 45.0 * t587 * t7687;
    let t7690 = t1000 * t1406;
    let t7691 = t1821 * t7690;
    let t7693 = 8.0 / 45.0 * t1820 * t7691;
    let t7694 = t2620 * t197;
    let t7695 = t7694 * t7355;
    let t7697 = 32.0 / 45.0 * t587 * t7695;
    let t7698 = t331 * t1660;
    let t7699 = t7698 * t197;
    let t7700 = t7699 * t7346;
    let t7702 = 16.0 / 27.0 * t587 * t7700;
    let t7703 = t1885 * t1802;
    (t7689, t7693, t7694, t7697, t7702, t7703)
}
