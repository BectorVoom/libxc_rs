//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta223 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1076;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1077;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta223(t1230: f64, t248: f64, t4733: f64, t3440: f64, t4724: f64, t1193: f64, t1706: f64, t135: f64, t1725: f64, t1174: f64, t1196: f64, t3966: f64, t974: f64, t1198: f64, t1213: f64, t1218: f64, t1227: f64, t1232: f64, t1748: f64, t3490: f64, t3524: f64, t3542: f64, t3543: f64, t3547: f64, t3549: f64, t3573: f64, t4889: f64, t5014: f64, t5019: f64, t5024: f64, t5010: f64, t466: f64, t1752: f64, t225: f64, t1251: f64, t1760: f64, t3598: f64, t1243: f64, t5000: f64, t1215: f64, t3612: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5030, t5033, t5036, t5041, t5045) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1076(t1230, t248, t4733, t3440, t4724, t1193, t1706, t135, t1725, t1174, t1196, t3966);
        let t5051 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1077(t5045, t974, t1174, t1198, t1213, t1218, t1227, t1232, t1748, t3490, t3524, t3542, t3543, t3547, t3549, t3573, t4889, t5014, t5019, t5024, t5030, t5033, t5036, t5041);
        let (t5052, t5053, t5055, t5060, t5064, t5068) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1078(t5010, t5051, t466, t1752, t225, t1251, t1760, t3598, t1243, t5000, t1215, t3612);
    (t5030, t5045, t5052, t5053, t5055, t5060, t5064, t5068)
}
