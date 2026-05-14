//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 887/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk887<F: Float>(t34794: F, t1095: F, t1980: F, t4806: F, t7476: F, t7799: F, t8555: F, t1530: F, t31056: F, t3073: F, t33953: F, t4241: F, t13364: F, t13299: F, t4349: F, t7741: F) -> (F, F, F, F, F, F, F) {
    let t34795 = 0.15724046144802076034e-2 * t34794;
    let t34802 = t1980 * t7476 * t1095 * t4806;
    let t34803 = 0.10482697429868050689e-2 * t34802;
    let t34804 = t7799 * t8555;
    let t34823 = t1530 * t31056;
    let t34833 = t3073 * t31056;
    let t34834 = t33953 * t4241;
    let t34836 = t34833 * t13364 * t34834;
    let t34837 = 0.42874018118069736972e-3 * t34836;
    let t34839 = t34833 * t13299 * t34834;
    let t34840 = 0.62896184579208304136e-3 * t34839;
    let t34844 = t7741 * t4349;
    (t34795, t34803, t34804, t34823, t34837, t34840, t34844)
}
