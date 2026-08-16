//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta314 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1338;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta314(t154: f64, t3584: f64, t3241: f64, t636: f64, t52: f64, t1094: f64, t3312: f64, t3311: f64, t419: f64, t409: f64, t11135: f64, t10292: f64, t281: f64, t415: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11145, t11147, t11153, t11185, t11190, t11195, t11203) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1338(t154, t3584, t3241, t636, t52, t1094, t3312, t3311, t419, t409, t11135, t10292, t281, t415);
    (t11145, t11147, t11153, t11185, t11190, t11195, t11203)
}
