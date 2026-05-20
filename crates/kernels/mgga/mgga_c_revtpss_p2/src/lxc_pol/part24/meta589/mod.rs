//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1848;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta589<F: Float>(t87071: F, t92516: F, t116: F, t117: F, t1916: F, t1918: F, t22633: F, t25055: F, t25063: F, t25066: F, t25069: F, t572: F, t573: F, t5801: F, t5883: F, t5920: F, t6941: F, t6945: F, t6948: F, t87051: F, t87237: F, param_d: F, t1458: F, t1914: F, t1921: F, t25049: F, t25072: F, t3: F, t575: F, t6937: F, t6951: F, t75808: F, t86897: F, t86903: F, t86909: F) -> F {
        let (t92517, t92552) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1848::<F>(t87071, t92516, t116, t117, t1916, t1918, t22633, t25055, t25063, t25066, t25069, t572, t573, t5801, t5883, t5920, t6941, t6945, t6948, t87051, t87237, param_d);
        let tv4rho44 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1849::<F>(t1458, t1914, t1921, t25049, t25072, t3, t575, t6937, t6951, t75808, t86897, t86903, t86909, t92517, t92552);
    tv4rho44
}
