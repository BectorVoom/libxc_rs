//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 827/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk827<F: Float>(t3236: F, t3245: F, t1032: F, t2689: F, t1001: F, t3271: F, t982: F, t12652: F, t12654: F, t12656: F, t12660: F, t12665: F, t12667: F, t12669: F, t12672: F, t12675: F, t12678: F, t12683: F) -> (F, F, F, F) {
    let t12685 = t3236 * t3245;
    let t12687 = t1032 * t2689;
    let t12689 = t3271 * t1001;
    let t12690 = t982 * t12689;
    let t12692 = t12652 / F::cast_from(8.0_f64) - F::cast_from(3.0_f64) * t12654 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t12656 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t12660 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t12665 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t12667 - F::cast_from(15.0_f64) / F::cast_from(16.0_f64) * t12669 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t12672 - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t12675 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t12678 + F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t12683 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t12685 + F::cast_from(9.0_f64) / F::cast_from(4.0_f64) * t12687 + F::cast_from(15.0_f64) / F::cast_from(16.0_f64) * t12690;
    (t12685, t12687, t12690, t12692)
}
