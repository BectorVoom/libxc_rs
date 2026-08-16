//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1829;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta558(t1404: f64, t7222: f64, t24447: f64, t580: f64, t2098: f64, t3946: f64, t1395: f64, t7240: f64, t1453: f64, t81439: f64, t26129: f64, t81442: f64, t22470: f64, t4067: f64, t2332: f64, t81446: f64, t666: f64, t22473: f64, t2358: f64, t12808: f64, t6530: f64, t12816: f64, t191: f64, t192: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t85381, t85392, t85394, t85397, t86586, t86588) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1829(t1404, t7222, t24447, t580, t2098, t3946, t1395, t7240, t1453, t81439, t26129, t81442);
        let (t86590, t86593, t86596, t86599, t86601, t86672) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1830(t22470, t4067, t1453, t2332, t81446, t666, t22473, t2358, t12808, t6530, t12816, t191, t192);
    (t85381, t85392, t85394, t85397, t86586, t86588, t86590, t86593, t86596, t86599, t86601, t86672)
}
