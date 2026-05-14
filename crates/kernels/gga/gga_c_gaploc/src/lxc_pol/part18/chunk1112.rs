//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1112/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1112<F: Float>(t10704: F, t1850: F, t10636: F, t5227: F, t1841: F, t3487: F, t7275: F, t734: F, t10826: F, t2536: F, t1944: F, t3444: F, t29478: F, t29480: F, t29483: F, t29486: F, t29489: F, t32604: F, t32610: F, t32615: F, t32618: F, t32621: F) -> (F,) {
    let t32622 = t1850 * t10704;
    let t32623 = 0.85450291446024714264e-3 * t32622;
    let t32625 = 0.17090058289204942853e-2 * t5227 * t10636;
    let t32629 = 0.17090058289204942853e-2 * t1841 * t7275 * t3487 * t734;
    let t32633 = 0.17090058289204942853e-2 * t1841 * t2536 * t10826 * t734;
    let t32634 = t1944 * t3444;
    let t32635 = 0.99692006687028833308e-3 * t32634;
    let t32636 = -t32604 - t32610 + t32615 + t32618 + t32621 - t32623 - t32625 - t32629 - t32633 - t29478 + t29480 + t29483 + t29486 - t29489 - t32635;
    (t32636,)
}
