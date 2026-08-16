//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1901;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1902;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta600<F: Float>(t22635: F, t26331: F, t26337: F, t90506: F, t26216: F, t81159: F, t26210: F, t6897: F, t794: F, t1377: F, t5187: F, t1385: F, t22633: F, t7692: F, t81186: F, t26338: F, t81228: F, t81326: F, t6888: F, t7691: F, t80707: F, t22666: F, t26189: F, t22892: F, t80645: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t90509, t90511, t90514, t90519) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1901::<F>(t22635, t26331, t26337, t90506, t26216, t81159, t26210, t6897, t794, t1377, t5187, t1385, t22633);
        let (t90521, t90524, t90527, t90530, t90533) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1902::<F>(t7692, t81186, t26338, t81228, t81326, t6888, t7691, t80707, t22666, t26189, t22892, t80645);
    (t90509, t90511, t90514, t90519, t90521, t90524, t90527, t90530, t90533)
}
