//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 916/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk916<F: Float>(t13506: F, t2087: F, t4614: F, t43715: F, t10931: F, t23220: F, t45316: F, t13559: F, t1445: F, t1991: F, t2615: F, t326: F, t3431: F, t43712: F, t43718: F, t45337: F, t45513: F, t45517: F, t45520: F, t45522: F, t45527: F, t45530: F, t45536: F, t45543: F, t45549: F, t45553: F, t45557: F, t45560: F, t590: F, t813: F, t8528: F) -> F {
    let t45563 = F::cast_from(0.82820720060468819301e2_f64) * t2087 * t4614 * t13506;
    let t45565 = F::cast_from(0.23833659967900284446e0_f64) * t43715;
    let t45569 = F::cast_from(0.27606906686822939767e2_f64) * t23220 * t10931 * t45316;
    let t45570 = -F::cast_from(0.57514388930881124515e0_f64) * t45513 - t45517 + t45520 + F::cast_from(0.76685851907841499353e0_f64) * t45522 - t45527 + t45530 + F::cast_from(0.92023022289409799224e1_f64) * t2615 * t326 * t45337 + t45536 - F::cast_from(0.92023022289409799224e1_f64) * t813 * t1445 * t8528 * t3431 - t45543 + F::cast_from(0.51123901271894332902e0_f64) * t1991 * t13559 * t590 - t45549 - t45553 - t45557 - t45560 - t45563 + F::cast_from(0.76685851907841499353e0_f64) * t43712 - t45565 + F::cast_from(0.31952438294933958064e0_f64) * t43718 - t45569;
    t45570
}
