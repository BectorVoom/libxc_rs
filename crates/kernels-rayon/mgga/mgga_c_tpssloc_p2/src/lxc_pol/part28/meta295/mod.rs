//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta295 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1205;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1206;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta295(t2250: f64, t2989: f64, t2775: f64, t343: f64, t2244: f64, t2987: f64, t3014: f64, t2262: f64, t972: f64, t2960: f64, t2971: f64, t2970: f64, t2995: f64, t973: f64, t2769: f64, t40: f64, t698: f64, t986: f64, t135: f64, t3010: f64, t241: f64, t625: f64, t281: f64, t283: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10245, t10254, t10255, t10259, t10263, t10267, t10273) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1205(t2250, t2989, t2775, t343, t2244, t2987, t3014, t2262, t972, t2960, t2971, t2970, t2995);
        let (t10274, t10277, t10287, t10290, t10292, t10294) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1206(t10273, t973, t2769, t40, t698, t986, t135, t3010, t241, t625, t281, t283);
    (t10245, t10254, t10255, t10259, t10263, t10267, t10274, t10277, t10287, t10290, t10292, t10294)
}
