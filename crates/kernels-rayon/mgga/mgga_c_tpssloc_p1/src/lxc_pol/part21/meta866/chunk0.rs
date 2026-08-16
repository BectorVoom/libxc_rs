//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3160/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3160(t65278: f64, t65279: f64, t65281: f64, t65282: f64, t65285: f64, t65286: f64, t65297: f64, t65327: f64, t11881: f64, t11907: f64, t1235: f64, t1244: f64, t1246: f64, t14986: f64, t15000: f64, t15009: f64, t15027: f64, t15239: f64, t1755: f64, t18940: f64, t19128: f64, t19160: f64, t19179: f64, t3610: f64, t3612: f64, t3613: f64, t3624: f64, t3626: f64, t491: f64, t5064: f64, t5079: f64, t6260: f64, t65221: f64, t65254: f64, t65262: f64, t65265: f64) -> (f64, f64) {
    let t65330 = t65278 + t65279 + t65281 + t65282 + t65285 + t65286 + t65297 + t65327;
    let t65343 = 2.0_f64 * t1235 * t1244 * t1246 * t18940 + t1244 * t1246 * t491 * t65330 + 4.0_f64 * t15239 * t1755 * t3610 * t3612 + 6.0_f64 * t11881 * t15000 * t6260 - 2.0_f64 * t19128 * t3624 * t5079 - 4.0_f64 * t19179 * t3624 * t5079 + 2.0_f64 * t3610 * t3612 * t65221 + 4.0_f64 * t3610 * t3612 * t65265 - 2.0_f64 * t11907 * t19160 + 2.0_f64 * t14986 * t5064 + 4.0_f64 * t15009 * t15027 + 2.0_f64 * t3613 * t65254 - t3626 * t65262;
    (t65330, t65343)
}
