//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1188;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1189;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta299<F: Float>(t221: F, t4019: F, t4057: F, t4018: F, t1386: F, t2681: F, t820: F, t1401: F, t4000: F, t843: F, t4006: F, t136: F, t4011: F, t3829: F, t3978: F, t3970: F, t3989: F, t4056: F, t550: F, t543: F, t3992: F, t2661: F, t240: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9905, t9906, t9909, t9910, t9919, t9921) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1188::<F>(t221, t4019, t4057, t4018, t1386, t2681, t820, t1401, t4000, t843, t4006, t136, t4011);
        let (t9923, t9924, t9926, t9930, t9932, t9934) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1189::<F>(t221, t3829, t9921, t3978, t3970, t3989, t4056, t550, t543, t3992, t2661, t240, t4000);
    (t9905, t9906, t9909, t9910, t9919, t9921, t9923, t9924, t9926, t9930, t9932, t9934)
}
