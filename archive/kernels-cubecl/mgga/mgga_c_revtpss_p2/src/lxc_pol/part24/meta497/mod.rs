//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta497 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1497;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1498;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta497<F: Float>(t23384: F, t689: F, t779: F, t14987: F, t18797: F, t23388: F, t786: F, t789: F, t23414: F, t23413: F, t41070: F, t686: F, t72: F, t18805: F, t50208: F, t4321: F, t6049: F, t4481: F, t63084: F, t1580: F, t18316: F, t14480: F, t252: F, t2782: F, t6071: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t75950, t75956, t75961, t75974, t75978) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1497::<F>(t23384, t689, t779, t14987, t18797, t23388, t786, t789, t23414, t23413, t41070, t686, t72);
        let (t75984, t75998, t76010, t76020, t76026) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1498::<F>(t18805, t50208, t4321, t6049, t689, t4481, t63084, t1580, t18316, t14480, t252, t2782, t6071);
    (t75950, t75956, t75961, t75974, t75978, t75984, t75998, t76010, t76020, t76026)
}
