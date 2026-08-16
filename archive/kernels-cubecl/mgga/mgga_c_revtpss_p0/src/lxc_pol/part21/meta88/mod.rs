//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta88 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk623;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta88<F: Float>(t118: F, t1502: F, t1519: F, t1843: F, t1847: F, t1911: F, t508: F, t511: F, t569: F, t651: F, t3: F, t117: F, t1518: F, param_d: F, t572: F, t573: F, t76: F, t84: F, t198: F, t207: F) -> (F, F, F, F, F, F, F) {
        let (t1913, t1914, t1916, t1918) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk623::<F>(t118, t1502, t1519, t1843, t1847, t1911, t508, t511, t569, t651, t3, t117, t1518, param_d);
        let (t1921, t1927, t1940) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk624::<F>(t1916, t1918, t572, t573, t76, t84, t198, t207);
    (t1913, t1914, t1916, t1918, t1921, t1927, t1940)
}
