//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1049;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1050;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1051;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta284(t12156: f64, t210: f64, t214: f64, t1307: f64, t213: f64, t221: f64, t3719: f64, t116: f64, t547: f64, t212: f64, t2586: f64, t12012: f64, t535: f64, t9534: f64, t9538: f64, t12188: f64, t12190: f64, t12194: f64, t12196: f64, t12197: f64, t12200: f64, t12205: f64, t12209: f64, t12212: f64, t12215: f64, t1315: f64, t5195: f64, t225: f64, t3792: f64, t3850: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12217, t12220, t12222, t12225, t12226, t12227, t12228, t12231) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1049(t12156, t210, t214, t1307, t213, t221, t3719, t116, t547, t212, t2586, t12012);
        let t12237 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1050(t535, t9534, t9538, t12188, t12190, t12194, t12196, t12197, t12200, t12205, t12209, t12212, t12215, t12217, t12222, t12228, t12231, t1315, t5195);
        let (t12238, t12240) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1051(t12237, t225, t3792, t3850);
    (t12217, t12220, t12222, t12225, t12226, t12227, t12231, t12237, t12238, t12240)
}
