//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 667/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk667(t2165: f64, t228: f64, t2766: f64, t2771: f64, t7608: f64, t7610: f64, t7611: f64, t7614: f64, t7631: f64, t7655: f64, t7657: f64, t7660: f64, t7669: f64, t899: f64, t906: f64) -> f64 {
    let t7671 = -t2165 * t2766 + t228 * t7655 + 2.0_f64 * t2771 * t7660 - t7657 * t906 - t7669 * t899 - t7608 + t7610 + t7611 - t7614 + t7631;
    t7671
}
