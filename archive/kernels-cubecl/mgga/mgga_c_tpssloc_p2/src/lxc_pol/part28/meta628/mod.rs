//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1967;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1968;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta628<F: Float>(t1408: F, t1877: F, t2057: F, t23302: F, t24191: F, t24335: F, t25021: F, t25028: F, t2522: F, t26563: F, t26740: F, t26744: F, t26756: F, t47645: F, t606: F, t7110: F, t7545: F, t7809: F, t84791: F, t84797: F, t86707: F, t86714: F, t86727: F, t86771: F, t87953: F, t87978: F, t87988: F, t193: F, t7125: F, t26739: F, t2752: F, t200: F, t7109: F, t86755: F, t24339: F, t24344: F, t25015: F, t25375: F, t25377: F, t25381: F, t25392: F, t6671: F, t7114: F, t7475: F, t86764: F, t86794: F, t86806: F, t86810: F, t86830: F, t87957: F, t87961: F, t201: F, t7844: F, t2249: F, t22951: F, t22961: F, t22968: F, t23299: F, t25024: F, t25366: F, t4314: F, t7845: F, t86710: F, t86746: F, t86782: F, t86803: F, t86816: F, t86825: F, t87981: F, t87994: F) -> (F, F, F, F, F, F, F, F) {
        let t92270 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1967::<F>(t1408, t1877, t2057, t23302, t24191, t24335, t25021, t25028, t2522, t26563, t26740, t26744, t26756, t47645, t606, t7110, t7545, t7809, t84791, t84797, t86707, t86714, t86727, t86771, t87953, t87978, t87988);
        let (t92271, t92276, t92295, t92299, t92309) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1968::<F>(t193, t7125, t26739, t2752, t200, t7109, t24191, t86755, t1877, t2057, t24335, t24339, t24344, t25015, t2522, t25375, t25377, t25381, t25392, t26563, t26756, t6671, t7114, t7475, t86764, t86794, t86806, t86810, t86830, t87957, t87961);
        let (t92319, t92349) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1969::<F>(t193, t201, t7844, t1877, t2057, t2249, t22951, t22961, t22968, t23299, t24191, t25024, t2522, t25366, t26563, t26744, t4314, t7110, t7114, t7845, t84797, t86710, t86746, t86782, t86803, t86816, t86825, t87981, t87994);
    (t92270, t92271, t92276, t92295, t92299, t92309, t92319, t92349)
}
