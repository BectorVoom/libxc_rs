//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 907/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk907(t2365: f64, t35623: f64, t6111: f64, t35445: f64, t7290: f64, t43581: f64, t10667: f64, t13682: f64, t1445: f64, t2033: f64, t2949: f64, t43529: f64, t44772: f64, t45366: f64, t45367: f64, t45372: f64, t45375: f64, t45377: f64, t45379: f64, t45381: f64, t45383: f64, t45385: f64, t45387: f64, t45390: f64, t45392: f64, t45394: f64, t549: f64, t813: f64, t8478: f64) -> f64 {
    let t45396 = t6111 * t2365 * t35623;
    let t45397 = 0.29792074959875355558e-1_f64 * t45396;
    let t45407 = t6111 * t2365 * t7290 * t35445;
    let t45408 = 0.17875244975925213335e0_f64 * t45407;
    let t45411 = 0.25561950635947166451e0_f64 * t43581;
    let t45412 = t45366 + t45367 - 0.59584149919750711116e-1_f64 * t43529 - t45372 - t45375 - t45377 + t45379 + t45381 + t45383 - t45385 + t45387 - t45390 + t45392 + t45394 + t45397 - 0.92023022289409799224e1_f64 * t813 * t1445 * t2949 * t10667 + 0.39722766613167140743e-1_f64 * t2033 * t549 * t44772 + t45408 - 0.21450293971110256002e1_f64 * t8478 * t13682 - t45411;
    t45412
}
