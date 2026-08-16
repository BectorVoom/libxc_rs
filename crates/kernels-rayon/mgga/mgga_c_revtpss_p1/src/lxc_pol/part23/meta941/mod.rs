//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta941 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3091;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3092;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta941(t20645: f64, t57818: f64, t1149: f64, t12227: f64, t16668: f64, t6470: f64, t1189: f64, t1196: f64, t24407: f64, t3495: f64, t16676: f64, t6535: f64, t16784: f64, t6548: f64, t24494: f64, t3531: f64, t5181: f64, t6555: f64, t20896: f64, t5192: f64, t81352: f64, t81558: f64, t81560: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81562, t81566, t81570, t81573) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3091(t20645, t57818, t1149, t12227, t16668, t6470, t1189, t1196, t24407, t3495, t16676, t6535);
        let (t81575, t81577, t81580, t81582, t81583) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3092(t16784, t6548, t24494, t3531, t1196, t5181, t6555, t20896, t5192, t81352, t81558, t81560, t81562, t81566, t81570, t81573);
    (t81562, t81566, t81570, t81573, t81575, t81577, t81580, t81582, t81583)
}
