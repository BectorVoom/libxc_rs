//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 750/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk750<F: Float>(t1562: F, t4354: F, t592: F, t4357: F, t600: F, t1347: F, t3910: F, t1341: F, t3944: F, t11388: F, t473: F, t11536: F, t3918: F, t1559: F, t4330: F, t1563: F, t4323: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12729 = 1.0 / t4354 / t1562;
    let t12730 = t592 * t12729;
    let t12732 = 1.0 / t4357 / t600;
    let t12736 = t3910 * t1347;
    let t12741 = t1341 * t3944;
    let t12744 = t473 * t11388;
    let t12751 = t473 * t11536;
    let t12755 = t1341 * t3918;
    let t12761 = t1559 * t4330;
    let t12767 = t4323 * t1563;
    (t12730, t12732, t12736, t12741, t12744, t12751, t12755, t12761, t12767)
}
