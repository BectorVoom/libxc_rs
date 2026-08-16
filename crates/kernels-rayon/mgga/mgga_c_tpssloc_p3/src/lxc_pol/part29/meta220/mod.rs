//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1053;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1054;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta220(t457: f64, t4928: f64, t460: f64, t974: f64, t1184: f64, t1714: f64, t1174: f64, t1180: f64, t1187: f64, t3430: f64, t3433: f64, t3436: f64, t3447: f64, t4887: f64, t4889: f64, t4897: f64, t4901: f64, t4905: f64, t4909: f64, t4913: f64, t4917: f64, t4920: f64, t491: f64, t1235: f64, t1720: f64, t1721: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4929, t4930) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1053(t457, t4928, t460);
        let (t4934, t4935, t4936, t4940) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1054(t4930, t974, t457, t1184, t1714, t460, t1174, t1180, t1187, t3430, t3433, t3436, t3447, t4887, t4889, t4897, t4901, t4905, t4909, t4913, t4917, t4920);
        let (t4941, t4943, t4945) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1055(t491, t4940, t1235, t1720, t1721, t225);
    (t4929, t4930, t4934, t4935, t4936, t4940, t4941, t4943, t4945)
}
