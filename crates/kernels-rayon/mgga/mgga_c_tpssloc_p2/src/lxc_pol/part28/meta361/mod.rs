//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1347;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1348;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1349;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta361(t13546: f64, t908: f64, t136: f64, t4389: f64, t699: f64, t4386: f64, t10277: f64, t1409: f64, t2244: f64, t2826: f64, t4337: f64, t4339: f64, t690: f64, t4344: f64, t10564: f64, t13537: f64, t123: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13548, t13550, t13551, t13552, t13555, t13557, t13559, t13561, t13563) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1347(t13546, t908, t136, t4389, t699, t4386, t10277, t1409, t2244, t2826, t4337, t4339, t690);
        let t13566 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1348(t4344, t690);
        let (t13567, t13569) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1349(t13566, t10564, t13537, t123);
    (t13548, t13550, t13551, t13552, t13555, t13557, t13559, t13561, t13563, t13566, t13567, t13569)
}
