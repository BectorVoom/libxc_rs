//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta316 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1604;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1605;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta316<F: Float>(t1466: F, t2246: F, t2275: F, t4186: F, t580: F, t9342: F, t2282: F, t10389: F, t1469: F, t2299: F, t10398: F, t2306: F, t116: F, t4245: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13272, t13302, t13309, t13310, t13324, t13368, t13371, t13378, t13381) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1604::<F>(t1466, t2246, t2275, t4186, t580, t9342, t2282, t10389, t1469, t2299, t10398, t2306);
        let t13426 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1605::<F>(t116, t4245);
    (t13272, t13302, t13309, t13310, t13324, t13368, t13371, t13378, t13381, t13426)
}
