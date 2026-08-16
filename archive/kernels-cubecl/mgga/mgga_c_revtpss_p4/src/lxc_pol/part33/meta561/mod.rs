//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1957;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1958;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta561<F: Float>(t30128: F, t651: F, t18245: F, t1936: F, t1501: F, t1518: F, t4248: F, t7741: F, t5920: F, t93: F, t7889: F, t1312: F, t30004: F, t1937: F, t7735: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t30130, t30137, t30138) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1957::<F>(t30128, t651, t18245, t1936, t1501, t1518);
        let (t30140, t30142, t30143, t30145, t30147, t30149, t30154, t30156, t30158) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1958::<F>(t1936, t30138, t4248, t7741, t5920, t93, t7889, t1312, t30004, t18245, t1937, t7735);
    (t30130, t30137, t30138, t30140, t30142, t30143, t30145, t30147, t30149, t30154, t30156, t30158)
}
