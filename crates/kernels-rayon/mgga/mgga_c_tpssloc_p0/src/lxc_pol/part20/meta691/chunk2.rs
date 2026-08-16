//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2625/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2625(t11835: f64, t4889: f64, t1174: f64, t1725: f64, t2402: f64, t11665: f64, t11668: f64, t11692: f64, t11845: f64, t11850: f64, t1227: f64, t14730: f64, t14748: f64, t15654: f64, t15708: f64, t15710: f64, t3577: f64, t3578: f64, t45250: f64, t4582: f64, t4723: f64, t48554: f64, t52532: f64, t52538: f64, t53144: f64) -> f64 {
    let t53433 = t4889 * t11835;
    let t53434 = t53433 / 162.0_f64;
    let t53440 = t1174 * t2402 * t1725;
    let t53446 = 5.0_f64 / 768.0_f64 * t3577 * t11668 * t14730 * t52538 - 5.0_f64 / 4608.0_f64 * t11692 * t11668 * t4723 * t53144 - t11665 * t15710 / 384.0_f64 - t3577 * t3578 * t14748 * t15708 / 384.0_f64 + 5.0_f64 / 768.0_f64 * t1227 * t4582 * t15654 * t48554 - t45250 - t53434 + t4889 * t11845 / 108.0_f64 + t4889 * t11850 / 18.0_f64 - 5.0_f64 / 3888.0_f64 * t53440 + 5.0_f64 / 4608.0_f64 * t3577 * t11668 * t4723 * t52532;
    t53446
}
