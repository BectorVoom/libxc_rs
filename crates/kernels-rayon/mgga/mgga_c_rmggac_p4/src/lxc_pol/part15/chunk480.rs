//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 480/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk480(t53: f64, t1797: f64, t983: f64, t1375: f64, t280: f64, t437: f64, t5860: f64, t6042: f64, t815: f64, t1802: f64, t4408: f64, t1805: f64, t990: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t54 = t53 <= zeta_threshold;
    let t6047 = t983 * t1797;
    let t6053 = piecewise3(t54, 0.0_f64, 8.0_f64 / 27.0_f64 * t6042 * t280 - 8.0_f64 / 9.0_f64 * t1375 * t815 - 2.0_f64 / 9.0_f64 * t6047 * t280 + 2.0_f64 / 3.0_f64 * t437 * t5860);
    let t6054 = t4408 * t1802;
    let t6059 = t990 * t1805;
    (t6053, t6054, t6059)
}
