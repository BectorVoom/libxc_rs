//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1204/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1204(t1880: f64, t32792: f64, t225: f64, t258: f64, t7510: f64, t214: f64, t1484: f64, t30622: f64, t23270: f64, t22986: f64, t30676: f64, t6637: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32794 = 0.16449340668482264365e-1_f64 * t1880 * t32792;
    let t32808 = t7510 * t225 * t258;
    let t32809 = t214 * t32808;
    let t32811 = 0.16449340668482264365e-1_f64 * t1880 * t32809;
    let t32814 = t30622 * t1484;
    let t32815 = t23270 * t32814;
    let t32817 = 0.3289868133696452873e-1_f64 * t22986 * t32815;
    let t32818 = t30676 * t1484;
    let t32819 = t6637 * t32818;
    (t32794, t32808, t32809, t32811, t32814, t32815, t32817, t32818, t32819)
}
