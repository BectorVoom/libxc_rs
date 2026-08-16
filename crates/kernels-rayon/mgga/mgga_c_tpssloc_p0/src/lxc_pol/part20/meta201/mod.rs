//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta201 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1207;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1208;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1209;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1210;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta201(t4729: f64, t4908: f64, t1178: f64, t3966: f64, t1177: f64, t135: f64, t1716: f64, t1174: f64, t1714: f64, t3448: f64, t3451: f64, t3295: f64, t3464: f64, t4770: f64, t4773: f64, t4776: f64, t4779: f64, t457: f64, t460: f64, t974: f64, t1184: f64, t1180: f64, t1187: f64, t3430: f64, t3433: f64, t3436: f64, t3447: f64, t4887: f64, t4889: f64, t4897: f64, t4901: f64, t4905: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t4909, t4912, t4913, t4917, t4919) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1207(t4729, t4908, t1178, t3966, t1177, t135, t1716, t1174, t1714, t3448);
        let (t4920, t4928) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1208(t3451, t4919, t3295, t3464, t4770, t4773, t4776, t4779);
        let (t4930, t4931, t4934) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1209(t457, t4928, t460, t974);
        let (t4936, t4940) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1210(t1184, t1714, t460, t4934, t1174, t1180, t1187, t3430, t3433, t3436, t3447, t4887, t4889, t4897, t4901, t4905, t4909, t4913, t4917, t4920, t4931);
    (t4912, t4919, t4928, t4930, t4934, t4936, t4940)
}
