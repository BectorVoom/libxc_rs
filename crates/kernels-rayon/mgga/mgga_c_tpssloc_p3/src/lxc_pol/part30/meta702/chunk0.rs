//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2273/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2273(t23384: f64, t28681: f64, t1054: f64, t5943: f64, t1921: f64, t5914: f64, t6688: f64, t225: f64, t28505: f64, t28496: f64, t1066: f64, t17582: f64, t18165: f64, t23346: f64, t25406: f64, t25732: f64, t25757: f64, t25758: f64, t25826: f64, t28697: f64, t28713: f64, t3026: f64, t4557: f64, t6687: f64, t6691: f64, t6704: f64, t6705: f64, t82436: f64, t986: f64) -> (f64, f64) {
    let t99205 = t23384 * t28681;
    let t99209 = t1054 * t5943;
    let t99210 = t1921 * t99209;
    let t99214 = t6688 * t5914;
    let t99221 = t28505 * t225;
    let t99230 = t23384 * t28496;
    let t99238 = -0.54831135561607547883e-2_f64 * t99205 + t82436 + 2.0_f64 * t3026 * t28713 + 0.82246703342411321825e-2_f64 * t6687 * t986 * t99210 + 0.27415567780803773942e-2_f64 * t6687 * t99214 * t6691 - 0.16449340668482264365e-1_f64 * t6687 * t25406 * t25826 - t99221 * t1066 - 12.0_f64 * t25757 * t25758 * t17582 - 2.0_f64 * t4557 * t25732 - 0.43864908449286038307e-1_f64 * t23346 * t28496 + 0.54831135561607547883e-2_f64 * t99230 - 0.82246703342411321825e-2_f64 * t6687 * t6704 * t6705 * t18165 - 6.0_f64 * t3026 * t28697;
    (t99209, t99238)
}
