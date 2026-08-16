//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2003/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2003(t22828: f64, t80853: f64, t80855: f64, t22783: f64, t3872: f64, t1336: f64, t2690: f64, t6950: f64, t1369: f64, t22782: f64, t3777: f64, t3876: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t80857 = t80853 * t80855 * t22828;
    let t80859 = t22783 * t3872;
    let t80866 = t1336 * t6950 * t2690;
    let t80867 = t80866 * t1369;
    let t80869 = t3777 * t22782;
    let t80870 = t80869 * t1369;
    let t80872 = t22783 * t3876;
    (t80857, t80859, t80866, t80867, t80869, t80870, t80872)
}
