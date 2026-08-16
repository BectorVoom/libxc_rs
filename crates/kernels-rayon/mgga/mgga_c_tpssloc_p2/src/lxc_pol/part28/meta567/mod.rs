//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1844;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1845;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta567(t25041: f64, t87049: f64, t215: f64, t6581: f64, t252: f64, t81613: f64, t13224: f64, t23056: f64, t13352: f64, t25242: f64, t6579: f64, t25245: f64, t82031: f64, t25038: f64, t4282: f64, t6646: f64, t9647: f64, t25251: f64, t23012: f64, t7529: f64, t13380: f64, t22986: f64, t2647: f64, t13377: f64, t1880: f64, t1894: f64, t214: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87050, t87052, t87055, t87059, t87066, t87068) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1844(t25041, t87049, t215, t6581, t252, t81613, t13224, t23056, t13352, t25242, t6579, t25245, t82031);
        let (t87076, t87078, t87080, t87084, t87092) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1845(t25038, t4282, t6646, t9647, t25251, t87049, t23012, t7529, t13380, t22986, t2647, t13377, t1880, t1894, t214);
    (t87050, t87052, t87055, t87059, t87066, t87068, t87076, t87078, t87080, t87084, t87092)
}
