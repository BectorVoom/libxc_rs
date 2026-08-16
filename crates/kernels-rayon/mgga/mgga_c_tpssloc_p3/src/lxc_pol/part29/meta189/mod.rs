//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta189 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk979;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk980;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk981;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta189(t1474: f64, t67: f64, t758: f64, t2431: f64, t2532: f64, t2653: f64, t2417: f64, t2423: f64, t2426: f64, t2486: f64, t2518: f64, t2530: f64, t2537: f64, t2538: f64, t2665: f64, t225: f64, t4210: f64, t228: f64, t68: f64, t1484: f64, t845: f64, t776: f64, t4119: f64, t824: f64, t1504: f64, t1506: f64, t230: f64, t822: f64, t825: f64, t232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4211, t4213, t4214, t4215, t4216, t4217) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk979(t1474, t67, t758, t2431, t2532, t2653, t2417, t2423, t2426, t2486, t2518, t2530, t2537, t2538, t2665);
        let (t4219, t4225, t4226, t4227, t4230, t4233) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk980(t225, t4210, t4217, t228, t68, t1484, t845, t776, t4119, t824, t1504, t1506, t230, t822, t825);
        let t4234 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk981(t232, t4233);
    (t4211, t4213, t4214, t4215, t4216, t4219, t4225, t4226, t4227, t4230, t4233, t4234)
}
