//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1901;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1902;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta600(t22635: f64, t26331: f64, t26337: f64, t90506: f64, t26216: f64, t81159: f64, t26210: f64, t6897: f64, t794: f64, t1377: f64, t5187: f64, t1385: f64, t22633: f64, t7692: f64, t81186: f64, t26338: f64, t81228: f64, t81326: f64, t6888: f64, t7691: f64, t80707: f64, t22666: f64, t26189: f64, t22892: f64, t80645: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90509, t90511, t90514, t90519) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1901(t22635, t26331, t26337, t90506, t26216, t81159, t26210, t6897, t794, t1377, t5187, t1385, t22633);
        let (t90521, t90524, t90527, t90530, t90533) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1902(t7692, t81186, t26338, t81228, t81326, t6888, t7691, t80707, t22666, t26189, t22892, t80645);
    (t90509, t90511, t90514, t90519, t90521, t90524, t90527, t90530, t90533)
}
