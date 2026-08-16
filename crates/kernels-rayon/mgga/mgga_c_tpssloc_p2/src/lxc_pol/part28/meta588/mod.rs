//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1881;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1882;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta588(t23185: f64, t25045: f64, t82074: f64, t254: f64, t799: f64, t23270: f64, t2379: f64, t25039: f64, t87642: f64, t1880: f64, t23218: f64, t25224: f64, t6562: f64, t6572: f64, t86893: f64, t23171: f64, t23228: f64, t7488: f64, t214: f64, t4265: f64, t25055: f64, t81591: f64, t25217: f64, t6547: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87753, t87755, t87765, t87773) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1881(t23185, t25045, t82074, t254, t799, t23270, t2379, t25039, t87642, t1880, t23218, t25224);
        let (t87776, t87779, t87782, t87784, t87786, t87796) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1882(t6562, t6572, t86893, t23171, t23228, t7488, t214, t4265, t1880, t25055, t81591, t25217, t6547);
    (t87753, t87755, t87765, t87773, t87776, t87779, t87782, t87784, t87786, t87796)
}
