//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 832/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk832<F: Float>(t385: F, t999: F, t247: F, t3116: F, t3140: F, t8507: F, t1078: F, t1982: F, t25669: F, t3268: F, t8513: F, t3143: F, t1043: F, t1089: F, t1976: F, t7150: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31921 = t385 * t999;
    let t31923 = t247 * t3116 * t31921;
    let t31926 = t8507 * t3140;
    let t31927 = t31926 * t1078;
    let t31928 = t1982 * t31927;
    let t31934 = t8513 * t25669 * t3268;
    let t31935 = t3143 * t8507;
    let t31937 = t31935 * t1043 * t1089;
    let t31940 = t1982 * t1976;
    let t31943 = t7150 * t8507;
    (t31921, t31923, t31926, t31927, t31928, t31934, t31935, t31937, t31940, t31943)
}
