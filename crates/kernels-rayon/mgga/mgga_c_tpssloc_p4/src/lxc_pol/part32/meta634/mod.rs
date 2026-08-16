//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta634 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2048;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta634(t87443: f64, t25064: f64, t81902: f64, t23077: f64, t6646: f64, t6590: f64, t23033: f64, t25155: f64, t6546: f64, t25112: f64, t81835: f64, t23083: f64, t25116: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t87444, t87445, t87447, t87451, t87464, t87478, t87487) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2048(t87443, t25064, t81902, t23077, t6646, t6590, t23033, t25155, t6546, t25112, t81835, t23083, t25116);
    (t87444, t87445, t87447, t87451, t87464, t87478, t87487)
}
