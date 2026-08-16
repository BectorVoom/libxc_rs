//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1031/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1031(t1992: f64, t22635: f64, t31558: f64, t6460: f64, t122166: f64, t6888: f64, t7691: f64, t102948: f64, t113963: f64, t12021: f64, t122178: f64, t122210: f64, t122297: f64, t127220: f64, t127229: f64, t127242: f64, t127249: f64, t127316: f64, t1375: f64, t1843: f64, t2015: f64, t2016: f64, t2092: f64, t28187: f64, t29360: f64, t33323: f64, t3887: f64, t6439: f64, t7194: f64, t8636: f64, t97558: f64, t97740: f64) -> f64 {
    let t128705 = t1992 * t22635 * t31558 * t6460;
    let t128724 = t6888 * t122166 * t7691;
    let t128726 = -t7194 * t28187 + 0.16449340668482264365e-1_f64 * t128705 + 2.0_f64 * t1375 * t3887 * t29360 * t2015 + t127220 - 0.16449340668482264365e-1_f64 * t122178 + t127229 - 12.0_f64 * t97740 * t33323 - t113963 - 2.0_f64 * t122297 * t1843 - t127242 - 6.0_f64 * t1375 * t12021 * t8636 * t6439 - t97558 * t2092 - t102948 * t2016 + t127249 + 0.38381794893125283518e-1_f64 * t122210 - 0.3289868133696452873e-1_f64 * t128724 + t127316;
    t128726
}
