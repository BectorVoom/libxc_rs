//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta387 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1417;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta387<F: Float>(t141: F, t2908: F, t41263: F, t11321: F, t698: F, t2297: F, t2851: F, t39443: F, t11341: F, t11331: F, t11144: F, t2439: F, t2912: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t41265, t41267, t41270, t41271, t41273, t41275, t41277, t41279, t41281) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1417::<F>(t141, t2908, t41263, t11321, t698, t2297, t2851, t39443, t11341, t11331, t11144, t2439, t2912);
    (t41265, t41267, t41270, t41271, t41273, t41275, t41277, t41279, t41281)
}
