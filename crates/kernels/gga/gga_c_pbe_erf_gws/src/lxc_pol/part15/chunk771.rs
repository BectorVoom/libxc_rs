//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 771/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk771<F: Float>(t6906: F, t4344: F, t4498: F, t19: F, t3025: F, t796: F, t801: F, t1105: F, t945: F, t810: F, t4545: F, t2474: F, t460: F, t40: F, t4757: F, t950: F) -> (F, F, F, F, F, F, F, F) {
    let t6907 = 0.6846054806677777778e0 * t6906;
    let t6911 = 0.41076328840066666668e0 * t4344;
    let t6918 = 4.0 * t4498;
    let t6921 = t3025 * t796 * t19;
    let t6922 = t6921 * t801;
    let t6923 = 0.82152657680133333336e0 * t6922;
    let t6925 = t945 * t1105;
    let t6926 = t6925 * t810;
    let t6929 = 0.12654485932329694421e1 * t4545;
    let t6930 = t2474 * t460;
    let t6931 = t40 * t6930;
    let t6932 = 2.0 * t6931;
    let t6933 = t4757 * t950;
    (t6907, t6911, t6918, t6923, t6926, t6929, t6932, t6933)
}
