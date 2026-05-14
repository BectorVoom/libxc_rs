//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 635/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk635<F: Float>(t10488: F, t4726: F, t26: F, t10442: F, t1659: F, t1660: F, t2877: F, t4733: F, t827: F, t4727: F, t5005: F, t79: F, t10464: F, t10450: F, t10570: F, t10572: F, t10574: F, t10576: F, t10587: F, t10595: F, t10607: F) -> (F, F, F, F, F, F, F, F) {
    let t10609 = t4726 * t10488;
    let t10610 = t26 * t10609;
    let t10612 = t1659 * t10442;
    let t10613 = t26 * t10612;
    let t10615 = t2877 * t1660;
    let t10617 = t827 * t4733;
    let t10619 = t827 * t4727;
    let t10621 = t79 * t5005;
    let t10622 = t10621 * t10464;
    let t10623 = t26 * t10622;
    let t10625 = t1659 * t10450;
    let t10626 = t26 * t10625;
    let t10634 = -0.33114e0 * t10607 + 0.16557e0 * t10610 - 0.49671e0 * t10613 - 0.27595e0 * t10615 + 0.16557e0 * t10617 + 0.5519e-1 * t10619 - 0.36793333333333333333e-1 * t10623 - 0.82785e-1 * t10626 - 0.60384999999999999999e0 * t10587 + 0.181155e1 * t10595 - 0.40256666666666666668e0 * t10570 + 0.20128333333333333333e0 * t10572 - 0.60385000000000000001e0 * t10574 + 0.30192500000000000001e0 * t10576;
    (t10610, t10613, t10615, t10617, t10619, t10623, t10626, t10634)
}
