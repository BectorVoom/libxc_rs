//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1262/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1262<F: Float>(t34407: F, t6710: F, t6711: F, t1429: F, t2365: F, t2366: F, t7861: F, t18970: F, t3381: F, t1: F, t1559: F, t544: F, t986: F, t6734: F, t204: F, t34246: F, t587: F) -> (F, F, F, F, F) {
    let t35192 = 0.23005755572352449806e2 * t6710 * t6711 * t34407;
    let t35198 = t1429 * t2365 * t2366 * t7861;
    let t35199 = 0.14896037479937677779e-1 * t35198;
    let t35200 = t18970 * t3381;
    let t35201 = 0.14896037479937677779e-1 * t35200;
    let t35204 = t544 * t1559 * t986 * t1;
    let t35206 = 0.21450293971110256001e2 * t35204 * t6734;
    let t35209 = 0.92023022289409799224e1 * t587 * t204 * t34246;
    (t35192, t35199, t35201, t35206, t35209)
}
