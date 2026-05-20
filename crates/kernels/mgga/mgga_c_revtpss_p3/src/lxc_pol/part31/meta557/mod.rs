//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1966;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta557<F: Float>(t29991: F, t30159: F, t3: F, t2042: F, t6941: F, t1916: F, t7950: F, t7953: F, t1936: F, t5883: F, t572: F, t1518: F, t28276: F, param_d: F, t5920: F, t7330: F, t117: F, t30004: F, t1918: F, t2040: F, t573: F, t6945: F, t6948: F, t7944: F) -> (F, F, F, F, F, F, F, F) {
        let (t30160, t30161, t30171, t30180, t30182, t30184, t30185, t30187, t30188) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1966::<F>(t29991, t30159, t3, t2042, t6941, t1916, t7950, t7953, t1936, t5883, t572, t1518, t28276, param_d);
        let (t30191, t30194, t30197) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1967::<F>(t30188, t572, t5920, t7330, t117, t30004, t1918, t2040, t30171, t30180, t30182, t30184, t30187, t573, t6945, t6948, t7944);
    (t30160, t30161, t30171, t30185, t30188, t30191, t30194, t30197)
}
