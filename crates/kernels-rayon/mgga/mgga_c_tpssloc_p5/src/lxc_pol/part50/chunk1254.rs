//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1254/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1254(t3886: f64, t7749: f64, t1385: f64, t1992: f64, t22635: f64, t1985: f64, t8458: f64, t90739: f64, t114187: f64, t114178: f64, t114194: f64, t120297: f64, t120304: f64, t120309: f64, t120312: f64, t120313: f64, t120316: f64, t1375: f64, t16022: f64, t1843: f64, t26371: f64, t26482: f64, t31131: f64, t3887: f64, t5215: f64, t6958: f64, t6992: f64, t8486: f64) -> f64 {
    let t120317 = t3886 * t7749;
    let t120321 = 0.3289868133696452873e-1_f64 * t1992 * t22635 * t120317 * t1385;
    let t120324 = 0.16449340668482264365e-1_f64 * t1985 * t90739 * t8458;
    let t120327 = 0.82246703342411321825e-2_f64 * t114187;
    let t120328 = 4.0_f64 * t1375 * t3887 * t6992 * t7749 - t114194 * t1843 - t16022 * t8486 + 4.0_f64 * t26371 * t6958 + 4.0_f64 * t26482 * t6958 + 2.0_f64 * t31131 * t5215 - t114178 + t120297 + t120304 + t120309 - t120312 + t120313 - t120316 + t120321 - t120324 + t120327;
    t120328
}
