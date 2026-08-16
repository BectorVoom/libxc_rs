//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta340 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1247;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1248;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta340(t13969: f64, t4584: f64, t1041: f64, t4589: f64, t2960: f64, t4603: f64, t1606: f64, t698: f64, t973: f64, t1043: f64, t2770: f64, t10277: f64, t3061: f64, t10216: f64, t10969: f64, t135: f64, t4608: f64, t10868: f64, t1539: f64, t248: f64, t1009: f64, t4552: f64, t1011: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14136, t14139, t14158, t14160, t14164, t14172) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1247(t13969, t4584, t1041, t4589, t2960, t4603, t1606, t698, t973, t1043, t2770, t10277, t3061);
        let (t14187, t14194, t14203, t14205, t14206) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1248(t10216, t10969, t135, t4608, t973, t10868, t1539, t248, t1041, t1009, t4552, t1011);
    (t14136, t14139, t14158, t14160, t14164, t14172, t14187, t14194, t14203, t14205, t14206)
}
