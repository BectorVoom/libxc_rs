//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1967;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1968;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta628(t1408: f64, t1877: f64, t2057: f64, t23302: f64, t24191: f64, t24335: f64, t25021: f64, t25028: f64, t2522: f64, t26563: f64, t26740: f64, t26744: f64, t26756: f64, t47645: f64, t606: f64, t7110: f64, t7545: f64, t7809: f64, t84791: f64, t84797: f64, t86707: f64, t86714: f64, t86727: f64, t86771: f64, t87953: f64, t87978: f64, t87988: f64, t193: f64, t7125: f64, t26739: f64, t2752: f64, t200: f64, t7109: f64, t86755: f64, t24339: f64, t24344: f64, t25015: f64, t25375: f64, t25377: f64, t25381: f64, t25392: f64, t6671: f64, t7114: f64, t7475: f64, t86764: f64, t86794: f64, t86806: f64, t86810: f64, t86830: f64, t87957: f64, t87961: f64, t201: f64, t7844: f64, t2249: f64, t22951: f64, t22961: f64, t22968: f64, t23299: f64, t25024: f64, t25366: f64, t4314: f64, t7845: f64, t86710: f64, t86746: f64, t86782: f64, t86803: f64, t86816: f64, t86825: f64, t87981: f64, t87994: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t92270 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1967(t1408, t1877, t2057, t23302, t24191, t24335, t25021, t25028, t2522, t26563, t26740, t26744, t26756, t47645, t606, t7110, t7545, t7809, t84791, t84797, t86707, t86714, t86727, t86771, t87953, t87978, t87988);
        let (t92271, t92276, t92295, t92299, t92309) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1968(t193, t7125, t26739, t2752, t200, t7109, t24191, t86755, t1877, t2057, t24335, t24339, t24344, t25015, t2522, t25375, t25377, t25381, t25392, t26563, t26756, t6671, t7114, t7475, t86764, t86794, t86806, t86810, t86830, t87957, t87961);
        let (t92319, t92349) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1969(t193, t201, t7844, t1877, t2057, t2249, t22951, t22961, t22968, t23299, t24191, t25024, t2522, t25366, t26563, t26744, t4314, t7110, t7114, t7845, t84797, t86710, t86746, t86782, t86803, t86816, t86825, t87981, t87994);
    (t92270, t92271, t92276, t92295, t92299, t92309, t92319, t92349)
}
