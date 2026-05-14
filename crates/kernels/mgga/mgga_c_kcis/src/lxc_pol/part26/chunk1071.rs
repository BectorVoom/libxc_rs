//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1071/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1071<F: Float>(t3716: F, t12229: F, t486: F, t506: F, t12344: F, t1502: F, t12343: F, t561: F, t588: F, t12857: F, t1588: F, t12856: F, t609: F, t625: F, t4313: F, t12938: F, t629: F, t632: F) -> (F, F, F, F, F, F, F, F) {
    let t38629 = t3716 * t3716;
    let t38630 = 1.0 / t38629;
    let t39052 = t486 / t12229 / t506;
    let t39301 = t1502 * t12344;
    let t39310 = t561 / t12343 / t588;
    let t40484 = t1588 * t12857;
    let t40512 = t609 / t12856 / t625;
    let t40514 = t4313 * t4313;
    let t40515 = 1.0 / t40514;
    let t40653 = t629 / t12938 / t632;
    (t38630, t39052, t39301, t39310, t40484, t40512, t40515, t40653)
}
