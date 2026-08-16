//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1300/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1300(t112: f64, t30465: f64, t100930: f64, t110240: f64, t111415: f64, t1401: f64, t1458: f64, t16521: f64, t16524: f64, t19534: f64, t20162: f64, t20173: f64, t20181: f64, t2180: f64, t29996: f64, t30180: f64, t30231: f64, t30250: f64, t30492: f64, t30495: f64, t3941: f64, t4072: f64, t5456: f64, t5493: f64, t55353: f64, t55388: f64, t66958: f64, t671: f64, t8143: f64, t8166: f64, t8230: f64, t8251: f64) -> f64 {
    let t111674 = t30465 * t112;
    let t111683 = 54.0_f64 * t55353 * t8251 + 27.0_f64 * t20173 * t30495 + 27.0_f64 * t3941 * t8143 * t5493 + 27.0_f64 * t3941 * t2180 * t19534 + 27.0_f64 * t110240 * t5456 + 0.135e2_f64 * t1401 * t111415 + 27.0_f64 * t30231 * t4072 + 54.0_f64 * t16524 * t30250 + 54.0_f64 * t20173 * t30492 + 54.0_f64 * t3941 * t30180 * t1458 + 54.0_f64 * t3941 * t8230 * t4072 + 27.0_f64 * t16521 * t8230 + 0.135e2_f64 * t66958 * t2180 + 27.0_f64 * t100930 * t2180 + 0.135e2_f64 * t111674 * t671 + 27.0_f64 * t29996 * t20181 + 0.135e2_f64 * t20162 * t8143 + 27.0_f64 * t55388 * t8166;
    t111683
}
