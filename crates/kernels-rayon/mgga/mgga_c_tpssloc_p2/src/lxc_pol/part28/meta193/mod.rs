//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta193 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk930;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk931;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk932;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk933;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk934;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta193(t1484: f64, t2523: f64, t2408: f64, t2417: f64, t2423: f64, t2426: f64, t2486: f64, t2518: f64, t2522: f64, t2530: f64, t2537: f64, t2538: f64, t2665: f64, t4209: f64, t4213: f64, t4214: f64, t4215: f64, t4216: f64, t4319: f64, t2: f64, t265: f64, t584: f64, t1540: f64, t690: f64, t1409: f64, t2770: f64, t607: f64, t2768: f64, t123: f64, t2775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4323 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk930(t1484, t2523, t2408, t2417, t2423, t2426, t2486, t2518, t2522, t2530, t2537, t2538, t2665, t4209, t4213, t4214, t4215, t4216);
        let t4324 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk931(t4319, t4323);
        let (t4331, t4332, t4335) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk932(t2, t265, t584, t1540, t690);
        let (t4337, t4338) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk933(t1409, t2770, t607);
        let (t4339, t4340, t4342, t4343) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk934(t2768, t4338, t123, t1409, t2775, t607);
    (t4324, t4331, t4332, t4335, t4337, t4338, t4339, t4340, t4342, t4343)
}
