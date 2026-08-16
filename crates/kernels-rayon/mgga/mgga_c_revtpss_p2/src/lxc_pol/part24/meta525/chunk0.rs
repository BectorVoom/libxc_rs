//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1556/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1556(t5326: f64, t6594: f64, t20973: f64, t5391: f64, t5381: f64, t12916: f64, t24735: f64, t5331: f64, t12855: f64, t24835: f64, t1038: f64, t1241: f64, t1244: f64, t24679: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t83114 = t5326 * t6594;
    let t83130 = t5391 * t20973;
    let t83136 = t5381 * t20973;
    let t83143 = t5331 * t12916 * t24735;
    let t83158 = t12855 * t12916 * t24835;
    let t83296 = t1241 * t1244 * t24679 * t1038;
    (t83114, t83130, t83136, t83143, t83158, t83296)
}
