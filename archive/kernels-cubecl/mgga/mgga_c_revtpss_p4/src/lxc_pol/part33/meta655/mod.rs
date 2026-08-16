//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2107;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2108;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta655<F: Float>(t29490: F, t571: F, t1459: F, t30188: F, t116: F, t30004: F, t572: F, t670: F, t1518: F, t1936: F, t4292: F, t6941: F, t7334: F, t30194: F, t21881: F, t7330: F, t1916: F, t28271: F, t28268: F, t30185: F, t25082: F, t86771: F, t8717: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t105804, t105818, t105822, t105826, t105830) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2107::<F>(t29490, t571, t1459, t30188, t116, t30004, t572, t670, t1518, t1936, t4292, t6941, t7334);
        let (t105834, t105837, t105839, t105841, t105843, t105859) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2108::<F>(t1459, t30194, t21881, t572, t7330, t1916, t28271, t28268, t30185, t25082, t86771, t8717);
    (t105804, t105818, t105822, t105826, t105830, t105834, t105837, t105839, t105841, t105843, t105859)
}
