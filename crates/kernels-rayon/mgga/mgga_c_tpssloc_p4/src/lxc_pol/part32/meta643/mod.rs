//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta643 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2061;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta643(t90514: f64, t1377: f64, t5187: f64, t7692: f64, t81186: f64, t26338: f64, t81228: f64, t81326: f64, t22892: f64, t7691: f64, t80645: f64, t26206: f64, t6883: f64, t1834: f64, t794: f64, t6891: f64, t22704: f64, t26355: f64, t26197: f64, t80670: f64, t213: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90515, t90516, t90521, t90525, t90534, t90541) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2061(t90514, t1377, t5187, t7692, t81186, t26338, t81228, t81326, t22892, t7691, t80645, t26206, t6883);
        let (t90542, t90544, t90547, t90550, t90551, t90566) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2062(t90541, t1834, t794, t22892, t6891, t22704, t26355, t81326, t26197, t80670, t213, t225);
    (t90515, t90516, t90521, t90525, t90534, t90542, t90544, t90547, t90550, t90551, t90566)
}
