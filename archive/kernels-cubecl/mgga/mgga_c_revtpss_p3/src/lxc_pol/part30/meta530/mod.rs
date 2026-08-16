//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta530 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1945;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta530<F: Float>(t4292: F, t93: F, t1936: F, t7002: F, t7889: F, t2322: F, t7741: F, t5523: F, t1312: F, t28042: F, t2042: F, t5795: F) -> (F, F, F, F, F, F, F) {
        let (t28219, t28221, t28223, t28225, t28227, t28229, t28257) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1945::<F>(t4292, t93, t1936, t7002, t7889, t2322, t7741, t5523, t1312, t28042, t2042, t5795);
    (t28219, t28221, t28223, t28225, t28227, t28229, t28257)
}
