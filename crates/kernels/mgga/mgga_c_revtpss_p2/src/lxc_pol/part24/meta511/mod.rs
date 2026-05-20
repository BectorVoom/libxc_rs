//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1526;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1527;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta511<F: Float>(t1063: F, t11725: F, t23481: F, t247: F, t23474: F, t3109: F, t23847: F, t3127: F, t3172: F, t23858: F, t23634: F, t1065: F, t24031: F, t11256: F, t23642: F, t23811: F, t300: F, t23470: F, t42534: F, t20050: F, t4834: F, t23843: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t78550, t78561, t78564, t78576, t78583, t78607) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1526::<F>(t1063, t11725, t23481, t247, t23474, t3109, t23847, t3127, t3172, t23858, t23634, t1065, t24031);
        let (t78676, t78704, t78750, t78756, t78763) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1527::<F>(t11256, t23642, t3172, t23811, t300, t1063, t23470, t247, t42534, t20050, t4834, t23843);
    (t78550, t78561, t78564, t78576, t78583, t78607, t78676, t78704, t78750, t78756, t78763)
}
