//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1349/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1349(t1858: f64, t8110: f64, t29865: f64, t580: f64, t2169: f64, t6483: f64, t29884: f64, t576: f64, t1390: f64, t20416: f64, t1983: f64, t6878: f64) -> (f64, f64, f64, f64, f64) {
    let t105144 = t8110 * t1858;
    let t105146 = t29865 * t580;
    let t105147 = t2169 * t6483;
    let t105150 = t576 * t29884;
    let t105159 = t1390 * t20416;
    let t105162 = 3.0_f64 * t1983 * t6878 * t105159;
    (t105144, t105146, t105147, t105150, t105162)
}
