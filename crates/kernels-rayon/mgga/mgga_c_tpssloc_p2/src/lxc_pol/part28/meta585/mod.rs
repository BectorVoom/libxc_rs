//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1875;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1876;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta585(t22893: f64, t23164: f64, t25320: f64, t1888: f64, t232: f64, t47528: f64, t6646: f64, t13398: f64, t82018: f64, t13404: f64, t22996: f64, t7521: f64, t81632: f64, t23035: f64, t2379: f64, t25319: f64, t6637: f64, t1887: f64, t81959: f64, t25248: f64, t25249: f64, t4265: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87618, t87627, t87630, t87633, t87635) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1875(t22893, t23164, t25320, t1888, t232, t47528, t6646, t13398, t82018, t13404, t22996, t7521, t81632);
        let (t87640, t87642, t87645, t87650) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1876(t23035, t2379, t25319, t6637, t1887, t81959, t25248, t25249, t1888, t232, t4265, t6646, t828);
    (t87618, t87627, t87630, t87633, t87635, t87640, t87642, t87645, t87650)
}
