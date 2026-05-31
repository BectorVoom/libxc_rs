//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3376/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3376<F: Float>(t63412: F, t63426: F, t63440: F, t63466: F, t923: F, t18979: F, t2889: F, t52035: F, t52037: F, t52039: F, t52041: F, t52045: F, t52047: F, t52049: F, t52051: F, t52065: F, t63393: F, t63396: F, t63399: F) -> (F, F, F, F) {
    let t63468 = t63412 + t63426 + t63440 + t63466;
    let t63469 = t923 * t63468;
    let t63471 = t18979 * t2889;
    let t63473 = F::cast_from(0.10629925925925925926e1_f64) * t52035 - F::cast_from(0.35433086419753086419e0_f64) * t52037 - F::cast_from(0.79724444444444444444e0_f64) * t52039 - F::cast_from(0.39862222222222222222e0_f64) * t52041 - F::cast_from(0.79724444444444444443e0_f64) * t52045 + F::cast_from(0.26574814814814814814e0_f64) * t52047 + F::cast_from(0.13287407407407407407e0_f64) * t52049 + F::cast_from(0.22145679012345679012e0_f64) * t52051 + F::cast_from(0.10954222222222222222e0_f64) * t52065 - F::cast_from(0.1460562962962962963e0_f64) * t63393 + F::cast_from(0.3071625e0_f64) * t63396 - F::cast_from(0.71752000000000000001e1_f64) * t63399 + F::cast_from(0.3071625e0_f64) * t63469 + F::cast_from(0.142419375e1_f64) * t63471;
    (t63468, t63469, t63471, t63473)
}
