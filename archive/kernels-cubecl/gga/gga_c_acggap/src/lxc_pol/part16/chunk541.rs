//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 541/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk541<F: Float>(t1004: F, t1244: F, t460: F, t848: F, t183: F, t3645: F, t188: F, t441: F, t862: F, t865: F, t447: F, t150: F) -> (F, F, F, F, F, F, F, F) {
    let t3842 = F::cast_from(0.19756347548806534796e1_f64) * t1004 * t1244;
    let t3843 = t848 * t460;
    let t3846 = F::cast_from(0.65854491829355115987e0_f64) * t3645 * t183;
    let t3862 = F::cast_from(0.65854491829355115987e0_f64) * t3645 * t188;
    let t3868 = t862 * t441;
    let t3869 = t3868 * t865;
    let t3873 = t447 * t447;
    let t3874 = F::cast_from(1.0_f64) / t3873;
    let t3875 = t150 * t3874;
    (t3842, t3843, t3846, t3862, t3869, t3873, t3874, t3875)
}
