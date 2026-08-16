//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1556;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1557;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta525(t5326: f64, t6594: f64, t20973: f64, t5391: f64, t5381: f64, t12916: f64, t24735: f64, t5331: f64, t12855: f64, t24835: f64, t1038: f64, t1241: f64, t1244: f64, t24679: f64, t21213: f64, t5357: f64, t1256: f64, t24681: f64, t24671: f64, t21233: f64, t1261: f64, t24240: f64, t247: f64, t3634: f64, t21192: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t83114, t83130, t83136, t83143, t83158, t83296) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1556(t5326, t6594, t20973, t5391, t5381, t12916, t24735, t5331, t12855, t24835, t1038, t1241, t1244, t24679);
        let (t83316, t83369, t83371, t83382, t83392, t83394) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1557(t21213, t5357, t1256, t24681, t24671, t21233, t5391, t1261, t24240, t247, t3634, t21192, t5381);
    (t83114, t83130, t83136, t83143, t83158, t83296, t83316, t83369, t83371, t83382, t83392, t83394)
}
