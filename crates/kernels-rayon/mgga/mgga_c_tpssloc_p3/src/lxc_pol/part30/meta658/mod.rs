//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta658 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2077;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta658(t90524: f64, t22892: f64, t7691: f64, t80645: f64, t26206: f64, t6883: f64, t1834: f64, t794: f64, t6891: f64, t22704: f64, t26355: f64, t81326: f64, t26197: f64, t80670: f64, t213: f64, t225: f64, t22724: f64, t26474: f64, t22751: f64, t26194: f64, t1887: f64, t80830: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90525, t90534, t90542, t90544, t90547, t90549) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2077(t90524, t22892, t7691, t80645, t26206, t6883, t1834, t794, t6891, t22704, t26355, t81326);
        let (t90550, t90551, t90566, t90582, t90585, t90591) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2078(t90549, t26197, t80670, t1834, t213, t225, t22724, t26474, t22751, t26194, t1887, t80830);
    (t90525, t90534, t90542, t90544, t90547, t90550, t90551, t90566, t90582, t90585, t90591)
}
