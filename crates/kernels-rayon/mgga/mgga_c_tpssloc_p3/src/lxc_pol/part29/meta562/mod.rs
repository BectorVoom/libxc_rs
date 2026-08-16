//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1969;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1970;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1971;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta562(t1251: f64, t8087: f64, t3598: f64, t225: f64, t497: f64, t5052: f64, t462: f64, t24574: f64, t8006: f64, t3242: f64, t3961: f64, t24601: f64, t24633: f64, t8002: f64, t254: f64, t492: f64, t11605: f64, t2154: f64, t5059: f64, t8055: f64, t2123: f64, t4930: f64, t1238: f64, t1252: f64, t14972: f64, t15820: f64, t1761: f64, t2121: f64, t2155: f64, t24646: f64, t24893: f64, t27549: f64, t3593: f64, t4945: f64, t5060: f64, t7283: f64, t7351: f64, t7356: f64, t8088: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27761, t27766, t27767, t27770, t27774, t27775, t27776) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1969(t1251, t8087, t3598, t225, t497, t5052, t462, t24574, t8006, t3242, t3961, t24601);
        let (t27779, t27784) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1970(t24633, t8002, t254, t492);
        let (t27785, t27786, t27792, t27794, t27797) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1971(t11605, t2154, t5059, t225, t8055, t2123, t4930, t1238, t1252, t14972, t15820, t1761, t2121, t2155, t24646, t24893, t27549, t27761, t27767, t27770, t27776, t27779, t27784, t3593, t4945, t5060, t7283, t7351, t7356, t8088);
    (t27761, t27766, t27774, t27775, t27776, t27779, t27784, t27785, t27786, t27792, t27794, t27797)
}
