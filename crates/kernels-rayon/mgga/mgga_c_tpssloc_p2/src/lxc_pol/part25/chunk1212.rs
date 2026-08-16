//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1212/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1212(t10049: f64, t10103: f64, t10116: f64, t2053: f64, t2054: f64, t24297: f64, t24330: f64, t2597: f64, t2718: f64, t2743: f64, t40852: f64, t40870: f64, t41554: f64, t7087: f64, t7107: f64, t82099: f64, t82108: f64, t84949: f64, t84981: f64, t85007: f64, t85031: f64, t855: f64, t858: f64) -> f64 {
    let t85047 = -3.0_f64 * t24297 * t2743 - 3.0_f64 * t40870 * t2054 + 0.15626873635058151147e0_f64 * t82099 + 6.0_f64 * t7087 * t10116 - t40852 * t2054 - t855 * t858 * (t84949 + t84981 + t85007 + t85031) + 6.0_f64 * t2597 * t24330 + 2.0_f64 * t855 * t2718 * t2053 * t10103 - 3.0_f64 * t10049 * t7107 - 3.0_f64 * t41554 * t2054 - 0.14804406601634037928e0_f64 * t82108;
    t85047
}
