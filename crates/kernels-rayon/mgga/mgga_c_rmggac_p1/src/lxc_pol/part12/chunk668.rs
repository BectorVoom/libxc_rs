//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 668/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk668(t2379: f64, t4041: f64, t2301: f64, t2604: f64, t1614: f64, t645: f64, t903: f64, t2127: f64, t534: f64, t72: f64, t7844: f64, t8642: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8879 = t4041 * t2379;
    let t8881 = t2604 * t2301;
    let t8884 = t645 * t1614;
    let t8885 = t903 * t8884;
    let t8887 = t534 * t2127;
    let t8888 = t72 * t8887;
    let t8889 = t7844 * t8642;
    (t8879, t8881, t8884, t8885, t8887, t8888, t8889)
}
