//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2193;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2194;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta629<F: Float>(t1497: F, t2311: F, t77: F, t4241: F, t640: F, t13420: F, t84: F, t10298: F, t1470: F, t2242: F, t4181: F, t4187: F, t28108: F, t644: F, t2315: F, t7705: F, t6977: F, t1927: F, t7719: F, t13272: F, t607: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t101172, t101176, t101182, t101187, t101190, t101193) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2193::<F>(t1497, t2311, t77, t4241, t640, t13420, t84, t10298, t1470, t2242, t4181, t4187);
        let (t101200, t101204, t101214, t101218, t101226, t101230) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2194::<F>(t28108, t644, t77, t2315, t7705, t1497, t6977, t1927, t4241, t7719, t13272, t607);
    (t101172, t101176, t101182, t101187, t101190, t101193, t101200, t101204, t101214, t101218, t101226, t101230)
}
