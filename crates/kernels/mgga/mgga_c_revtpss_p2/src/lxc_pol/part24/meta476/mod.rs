//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1460;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1461;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta476<F: Float>(t11262: F, t3127: F, t6262: F, t3160: F, t65338: F, t1062: F, t19463: F, t15711: F, t4834: F, t1041: F, t6301: F, t3150: F, t6307: F, t3201: F, t6318: F, t1011: F, t6292: F, t697: F, t19649: F, t372: F, t6284: F, t6288: F, t3091: F, t43240: F, t6267: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t65596, t65654, t65717, t65859, t66022, t66029) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1460::<F>(t11262, t3127, t6262, t3160, t65338, t1062, t19463, t15711, t4834, t1041, t6301, t3150, t6307);
        let (t66141, t66218, t66306, t66547, t66721, t66763) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1461::<F>(t3201, t6318, t1011, t6292, t697, t19649, t372, t6284, t6288, t3091, t43240, t6267);
    (t65596, t65654, t65717, t65859, t66022, t66029, t66141, t66218, t66306, t66547, t66721, t66763)
}
