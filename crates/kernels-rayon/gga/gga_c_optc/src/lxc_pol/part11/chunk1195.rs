//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1195/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1195(t12581: f64, t15983: f64, t15986: f64, t4492: f64, t19: f64, t54760: f64, t15889: f64, t4380: f64, t15597: f64, t4444: f64, t15843: f64, t4450: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54944 = t12581 * t15983;
    let t54947 = t4492 * t15986;
    let t54959 = t54760 * t19;
    let t54989 = t15889 * t4380;
    let t54999 = t4444 * t15597;
    let t55001 = t4450 * t15843;
    (t54944, t54947, t54959, t54989, t54999, t55001)
}
