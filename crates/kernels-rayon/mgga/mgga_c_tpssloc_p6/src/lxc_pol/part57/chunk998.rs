//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 998/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk998(t22986: f64, t33447: f64, t86873: f64, t1880: f64, t28294: f64, t31366: f64, t112936: f64, t112942: f64, t114933: f64, t114944: f64, t121405: f64, t126427: f64, t126497: f64, t127952: f64, t127955: f64, t127979: f64, t127990: f64, t127998: f64, t128020: f64, t1528: f64, t25168: f64, t25188: f64, t259: f64, t28310: f64, t33399: f64, t33452: f64, t4147: f64, t4268: f64, t5558: f64, t7842: f64, t8543: f64, t855: f64, t858: f64, t92394: f64) -> f64 {
    let t128035 = t22986 * t86873 * t33447;
    let t128040 = t1880 * t31366 * t28294;
    let t128042 = 4.0_f64 * t4147 * t33452 + 0.3289868133696452873e-1_f64 * t127952 - 0.16449340668482264365e-1_f64 * t127955 - t855 * t858 * (t127979 + t127990 + t127998 + t128020) - t126427 + t112936 - 2.0_f64 * t4268 * t33399 + t5558 * t8543 * t259 - t114933 - t112942 + 24.0_f64 * t25168 * t92394 * t28310 + t126497 - 2.0_f64 * t25188 * t7842 + 0.3289868133696452873e-1_f64 * t128035 - 2.0_f64 * t121405 * t1528 + 0.16449340668482264365e-1_f64 * t128040 + t114944;
    t128042
}
