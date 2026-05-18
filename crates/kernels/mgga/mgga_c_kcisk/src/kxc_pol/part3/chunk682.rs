//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 682/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk682<F: Float>(t4733: F, t827: F, t4727: F, t5005: F, t79: F, t10464: F, t26: F, t10450: F, t1659: F, t10570: F, t10572: F, t10574: F, t10576: F, t10587: F, t10595: F, t10607: F, t10610: F, t10613: F, t10615: F) -> (F, F, F, F, F) {
    let t10617 = t827 * t4733;
    let t10619 = t827 * t4727;
    let t10621 = t79 * t5005;
    let t10622 = t10621 * t10464;
    let t10623 = t26 * t10622;
    let t10625 = t1659 * t10450;
    let t10626 = t26 * t10625;
    let t10634 = -F::new(0.33114e0) * t10607 + F::new(0.16557e0) * t10610 - F::new(0.49671e0) * t10613 - F::new(0.27595e0) * t10615 + F::new(0.16557e0) * t10617 + F::new(0.5519e-1) * t10619 - F::new(0.36793333333333333333e-1) * t10623 - F::new(0.82785e-1) * t10626 - F::new(0.60384999999999999999e0) * t10587 + F::new(0.181155e1) * t10595 - F::new(0.40256666666666666668e0) * t10570 + F::new(0.20128333333333333333e0) * t10572 - F::new(0.60385000000000000001e0) * t10574 + F::new(0.30192500000000000001e0) * t10576;
    (t10617, t10619, t10623, t10626, t10634)
}
