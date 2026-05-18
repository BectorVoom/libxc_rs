//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 907/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk907<F: Float>(t2365: F, t35623: F, t6111: F, t35445: F, t7290: F, t43581: F, t10667: F, t13682: F, t1445: F, t2033: F, t2949: F, t43529: F, t44772: F, t45366: F, t45367: F, t45372: F, t45375: F, t45377: F, t45379: F, t45381: F, t45383: F, t45385: F, t45387: F, t45390: F, t45392: F, t45394: F, t549: F, t813: F, t8478: F) -> F {
    let t45396 = t6111 * t2365 * t35623;
    let t45397 = F::new(0.29792074959875355558e-1) * t45396;
    let t45407 = t6111 * t2365 * t7290 * t35445;
    let t45408 = F::new(0.17875244975925213335e0) * t45407;
    let t45411 = F::new(0.25561950635947166451e0) * t43581;
    let t45412 = t45366 + t45367 - F::new(0.59584149919750711116e-1) * t43529 - t45372 - t45375 - t45377 + t45379 + t45381 + t45383 - t45385 + t45387 - t45390 + t45392 + t45394 + t45397 - F::new(0.92023022289409799224e1) * t813 * t1445 * t2949 * t10667 + F::new(0.39722766613167140743e-1) * t2033 * t549 * t44772 + t45408 - F::new(0.21450293971110256002e1) * t8478 * t13682 - t45411;
    t45412
}
