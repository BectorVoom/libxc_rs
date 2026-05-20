//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1556;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1557;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta525<F: Float>(t5326: F, t6594: F, t20973: F, t5391: F, t5381: F, t12916: F, t24735: F, t5331: F, t12855: F, t24835: F, t1038: F, t1241: F, t1244: F, t24679: F, t21213: F, t5357: F, t1256: F, t24681: F, t24671: F, t21233: F, t1261: F, t24240: F, t247: F, t3634: F, t21192: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t83114, t83130, t83136, t83143, t83158, t83296) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1556::<F>(t5326, t6594, t20973, t5391, t5381, t12916, t24735, t5331, t12855, t24835, t1038, t1241, t1244, t24679);
        let (t83316, t83369, t83371, t83382, t83392, t83394) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1557::<F>(t21213, t5357, t1256, t24681, t24671, t21233, t5391, t1261, t24240, t247, t3634, t21192, t5381);
    (t83114, t83130, t83136, t83143, t83158, t83296, t83316, t83369, t83371, t83382, t83392, t83394)
}
