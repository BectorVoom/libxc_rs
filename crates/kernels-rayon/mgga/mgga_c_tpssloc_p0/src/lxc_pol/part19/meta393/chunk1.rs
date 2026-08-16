//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1492/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1492(t2342: f64, t100: f64, t103: f64, t2248: f64, t2336: f64, t2341: f64, t2343: f64, t2346: f64, t2349: f64, t2350: f64, t2354: f64, t45453: f64, t45460: f64, t45461: f64, t45469: f64, t45482: f64, t45496: f64, t657: f64, t660: f64, t92: f64, t9276: f64, t9374: f64, t9384: f64, t9386: f64, t9389: f64, t9390: f64, t9393: f64, t9394: f64, t9398: f64, t9403: f64, t9407: f64, t95: f64, t96: f64, tau0: f64) -> f64 {
    let t45497 = t2342 * t2342;
    let t45505 = 6160.0_f64 / 81.0_f64 * tau0 * t9276 * t96 + 10.0_f64 / 3.0_f64 * t92 * t2341 * t45453 + 40.0_f64 / 9.0_f64 * t92 * t9389 * t9393 + 40.0_f64 / 81.0_f64 * t100 * t45460 * t45461 - 20.0_f64 / 9.0_f64 * t100 * t9398 * t2350 * t2354 + 10.0_f64 / 3.0_f64 * t100 * t2349 * t45469 + 40.0_f64 / 9.0_f64 * t100 * t9403 * t9407 - 8800.0_f64 / 81.0_f64 * t9374 * t660 + 400.0_f64 / 9.0_f64 * t2336 * t2346 - 100.0_f64 / 9.0_f64 * t657 * t9394 + 5.0_f64 / 3.0_f64 * t92 * t95 * t45482 - 5.0_f64 / 3.0_f64 * t100 * t103 * t45482 + 800.0_f64 / 27.0_f64 * t2336 * t2343 + 200.0_f64 / 81.0_f64 * t657 * t9386 - 200.0_f64 / 9.0_f64 * t657 * t9390 + 40.0_f64 / 81.0_f64 * t92 * t45496 * t45497 - 20.0_f64 / 9.0_f64 * t92 * t9384 * t2342 * t2248;
    t45505
}
