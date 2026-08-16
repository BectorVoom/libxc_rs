//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta310 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1231;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1232;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1233;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1234;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1235;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta310(t11135: f64, t1091: f64, t2394: f64, t3244: f64, t690: f64, t3249: f64, t3253: f64, t154: f64, t3584: f64, t3241: f64, t636: f64, t52: f64, t1098: f64, t3256: f64, t1094: f64, t3312: f64, t3311: f64, t419: f64, t409: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11136, t11137) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1231(t11135, t1091, t2394);
        let t11139 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1232(t3244, t690);
        let t11141 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1233(t3249, t690);
        let t11143 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1234(t3253, t690);
        let (t11145, t11147, t11153, t11180, t11185, t11190) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1235(t154, t3584, t3241, t636, t52, t1098, t3256, t1094, t3312, t3311, t419, t409);
    (t11136, t11137, t11139, t11141, t11143, t11145, t11147, t11153, t11180, t11185, t11190)
}
