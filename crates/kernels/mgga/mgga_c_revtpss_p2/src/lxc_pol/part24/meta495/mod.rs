//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta495 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1495;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta495<F: Float>(t10069: F, t22361: F, t22365: F, t14239: F, t14242: F, t10023: F, t22314: F, t2470: F, t3999: F, t6888: F, t4086: F, t786: F) -> (F, F, F, F, F, F) {
        let (t75145, t75147, t75176, t75179, t75228, t75251) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1495::<F>(t10069, t22361, t22365, t14239, t14242, t10023, t22314, t2470, t3999, t6888, t4086, t786);
    (t75145, t75147, t75176, t75179, t75228, t75251)
}
