//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta139 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk722;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta139(t550: f64, t72: f64, t245: f64, t125: f64, t1882: f64, t1873: f64, t3957: f64, t1892: f64, t213: f64, t1357: f64, t1904: f64, t689: f64, t1903: f64, t686: f64, t3915: f64, t555: f64, t4086: f64, t543: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5673, t5674, t5681, t5715) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk722(t550, t72, t245, t125, t1882, t1873, t3957, t1892, t213);
        let (t5718, t5719, t5721, t5722, t5723, t5735, t5737) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk723(t1357, t1904, t689, t1903, t72, t686, t3915, t1882, t555, t4086, t543);
    (t5673, t5674, t5681, t5715, t5718, t5719, t5721, t5722, t5723, t5735, t5737)
}
