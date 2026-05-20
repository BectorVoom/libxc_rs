//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1987;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta539<F: Float>(t1936: F, t670: F, t1518: F, t572: F, t26123: F, t4292: F, t7330: F, t1459: F, t7953: F, t116: F, t7741: F, t117: F, t28042: F, t1461: F, t1918: F, t2040: F, t28246: F, t28257: F, t28259: F, t28261: F, t28263: F, t573: F, t5802: F, t5805: F, t7324: F, t7944: F) -> (F, F, F, F, F, F, F, F) {
        let (t28264, t28265, t28267, t28268, t28270, t28271, t28273, t28275, t28276, t28277, t28279, t28280) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1987::<F>(t1936, t670, t1518, t572, t26123, t4292, t7330, t1459, t7953, t116, t7741, t117, t28042);
        let t28283 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1988::<F>(t28280, t572, t1461, t1918, t2040, t28246, t28257, t28259, t28261, t28263, t28267, t28270, t28273, t28275, t28279, t573, t5802, t5805, t7324, t7944);
    (t28264, t28265, t28268, t28271, t28276, t28277, t28280, t28283)
}
