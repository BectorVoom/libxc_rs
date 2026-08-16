//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta188 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk918;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk919;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk920;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta188(t1742: f64, t372: f64, t479: f64, t471: f64, t1230: f64, t248: f64, t4733: f64, t3440: f64, t4724: f64, t1193: f64, t1706: f64, t135: f64, t1725: f64, t1174: f64, t1196: f64, t3966: f64, t974: f64, t1198: f64, t1213: f64, t1218: f64, t1227: f64, t1232: f64, t1748: f64, t3490: f64, t3524: f64, t3542: f64, t3543: f64, t3547: f64, t3549: f64, t3573: f64, t4889: f64, t5014: f64, t5019: f64, t5010: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5022, t5023, t5024) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk918(t1742, t372, t479, t471);
        let t5030 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk919(t1230, t248, t4733);
        let (t5033, t5036, t5040, t5041, t5045, t5046, t5051) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk920(t3440, t4724, t1193, t1706, t135, t1725, t1174, t1196, t3966, t974, t1198, t1213, t1218, t1227, t1232, t1748, t3490, t3524, t3542, t3543, t3547, t3549, t3573, t4889, t5014, t5019, t5024, t5030);
        let t5052 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk921(t5010, t5051);
    (t5022, t5023, t5024, t5030, t5033, t5036, t5040, t5041, t5045, t5046, t5052)
}
