//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta266 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1207;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1208;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1209;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1210;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1211;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1212;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1213;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta266(t1336: f64, t6944: f64, t1354: f64, t1358: f64, t2003: f64, t552: f64, t59: f64, t240: f64, t1369: f64, t6915: f64, t6917: f64, t6922: f64, t6929: f64, t6935: f64, t6938: f64, t6941: f64, t539: f64, t2007: f64, t225: f64, t1385: f64, t2015: f64, t3887: f64, t2010: f64, t6883: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6945 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1207(t1336, t6944);
        let (t6946, t6949, t6950, t6951) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1208(t1354, t6945, t1358, t2003, t552, t59, t240);
        let t6952 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1209(t1336, t6951);
        let t6955 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1210(t1369, t6952, t6915, t6917, t6922, t6929, t6935, t6938, t6941, t6946, t6949);
        let (t6956, t6958) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1211(t539, t6955, t2007, t225);
        let t6963 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1212(t1385, t2015, t3887);
        let (t6967, t6968) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1213(t2010, t6883, t552, t562);
    (t6945, t6949, t6950, t6951, t6952, t6955, t6956, t6958, t6963, t6967, t6968)
}
