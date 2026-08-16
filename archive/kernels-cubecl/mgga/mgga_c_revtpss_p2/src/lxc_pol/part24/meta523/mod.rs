//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta523 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1553;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1554;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta523<F: Float>(t1261: F, t12884: F, t24232: F, t247: F, t1263: F, t24616: F, t24633: F, t17525: F, t21188: F, t24758: F, t3172: F, t3711: F, t24643: F, t24770: F, t3153: F, t17569: F, t20783: F, t1222: F, t140: F, t24816: F, t24820: F, t12915: F, t24713: F, t5384: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t82757, t82799, t82816, t82821, t82824) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1553::<F>(t1261, t12884, t24232, t247, t1263, t24616, t24633, t17525, t21188, t24758, t3172, t3711);
        let (t82827, t82859, t82932, t82980, t82983, t83014) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1554::<F>(t1261, t24643, t3172, t24770, t3153, t17569, t20783, t1222, t140, t24816, t24820, t12915, t247, t24713, t5384);
    (t82757, t82799, t82816, t82821, t82824, t82827, t82859, t82932, t82980, t82983, t83014)
}
