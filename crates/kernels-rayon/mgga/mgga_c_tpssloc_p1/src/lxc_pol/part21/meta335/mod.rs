//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1712;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta335(t25: f64, t28: f64, t4021: f64, t645: f64, t1437: f64, t2307: f64, t1409: f64, t9321: f64, t2291: f64, t3966: f64, t584: f64, t9212: f64, zeta_threshold: f64, t9330: f64, t2298: f64, t2244: f64, t2250: f64, t4007: f64, t4012: f64, t607: f64, t634: f64, t638: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12585, t12588, t12595, t12598, t12603, t12604, t12606) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1712(t25, t28, t4021, t645, t1437, t2307, t1409, t9321, t2291, t3966, t584, t9212, zeta_threshold);
        let (t12609, t12619) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1713(t1409, t9330, t2298, t3966, t12595, t12598, t12606, t2244, t2250, t4007, t4012, t607, t634, t638);
    (t12585, t12588, t12595, t12603, t12604, t12606, t12609, t12619)
}
