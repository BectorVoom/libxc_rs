//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta182 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk837;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk838;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta182(t3451: f64, t4919: f64, t3295: f64, t3464: f64, t4770: f64, t4773: f64, t4776: f64, t4779: f64, t457: f64, t460: f64, t974: f64, t1184: f64, t1714: f64, t1174: f64, t1180: f64, t1187: f64, t3430: f64, t3433: f64, t3436: f64, t3447: f64, t4887: f64, t4889: f64, t4897: f64, t4901: f64, t4905: f64, t4909: f64, t4913: f64, t4917: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4920, t4928, t4930, t4931, t4934, t4935) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk837(t3451, t4919, t3295, t3464, t4770, t4773, t4776, t4779, t457, t460, t974, t1184, t1714);
        let (t4936, t4937, t4940) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk838(t460, t4935, t4934, t1174, t1180, t1187, t3430, t3433, t3436, t3447, t4887, t4889, t4897, t4901, t4905, t4909, t4913, t4917, t4920, t4931);
    (t4920, t4928, t4930, t4931, t4934, t4936, t4937, t4940)
}
