//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2286;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2287;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2288;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta655(t22666: f64, t26189: f64, t6888: f64, t22892: f64, t7691: f64, t80645: f64, t22633: f64, t22635: f64, t26337: f64, t3911: f64, t26206: f64, t6883: f64, t1834: f64, t794: f64, t6891: f64, t22704: f64, t26355: f64, t81326: f64, t26197: f64, t80670: f64, t1307: f64, t26331: f64, t5187: f64, t567: f64, t26332: f64, t3719: f64, t213: f64, t225: f64, t22637: f64, t26333: f64, t80650: f64, t16470: f64, t26224: f64, t26225: f64, t80689: f64, t80711: f64, t22724: f64, t26474: f64, t22751: f64, t26194: f64, t1887: f64, t80830: f64, t3734: f64, t22916: f64, t26193: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90530, t90534, t90539, t90541) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2286(t22666, t26189, t6888, t22892, t7691, t80645, t22633, t22635, t26337, t3911, t26206, t6883);
        let (t90542, t90544, t90547, t90550, t90551, t90556) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2287(t90541, t1834, t794, t22892, t6891, t22704, t26355, t81326, t26197, t80670, t1307, t22635, t26331, t5187, t567);
        let t90573 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2288(t22635, t26331, t26332, t3719, t1834, t213, t225, t22633, t22637, t26333, t80650, t16470, t26224, t26225, t80689, t90539, t90542, t90547, t90550, t90551, t90556);
        let (t90581, t90582, t90585, t90591, t90594, t90598) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2289(t80711, t22724, t26474, t22751, t26194, t1887, t80830, t22635, t26332, t3734, t22916, t26193, t6888);
    (t90530, t90534, t90544, t90573, t90581, t90582, t90585, t90591, t90594, t90598)
}
