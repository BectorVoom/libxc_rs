//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1033/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1033(t1985: f64, t29360: f64, t6889: f64, t6906: f64, t122142: f64, t1842: f64, t1992: f64, t22635: f64, t115551: f64, t115567: f64, t122172: f64, t122281: f64, t122295: f64, t127346: f64, t127349: f64, t127350: f64, t1807: f64, t1843: f64, t20044: f64, t26366: f64, t28111: f64, t29311: f64, t33266: f64, t33294: f64, t5321: f64, t568: f64, t6958: f64, t7194: f64, t7925: f64, t7937: f64, t8627: f64) -> f64 {
    let t128768 = t1985 * t6889 * t6906 * t29360;
    let t128781 = t1992 * t22635 * t122142 * t1842;
    let t128789 = t115551 - 2.0_f64 * t26366 * t7937 - 2.0_f64 * t5321 * t33294 - 0.82246703342411321825e-2_f64 * t128768 - t127346 + 4.0_f64 * t26366 * t7925 + 2.0_f64 * t7194 * t28111 + 4.0_f64 * t6958 * t29311 - 0.16449340668482264365e-1_f64 * t122281 + 2.0_f64 * t20044 * t8627 + 0.3289868133696452873e-1_f64 * t128781 - t127349 - t127350 + t115567 - 2.0_f64 * t122172 * t1843 + 2.0_f64 * t1807 * t33266 * t568 + 0.38381794893125283518e-1_f64 * t122295;
    t128789
}
