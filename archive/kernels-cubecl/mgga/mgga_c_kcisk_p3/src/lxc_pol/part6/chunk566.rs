//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 566/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk566<F: Float>(t3592: F, t7877: F, t457: F, t3611: F, t5668: F, t7738: F, t7742: F, t7746: F, t7758: F, t7765: F, t1355: F, t2083: F, t306: F, t3599: F, t5687: F, t7757: F, t7764: F) -> (F, F, F, F) {
    let t7878 = t3592 * t7877;
    let t7879 = t457 * t7878;
    let t7894 = -F::cast_from(0.991e-2_f64) * t7758 + F::cast_from(0.1982e-1_f64) * t7765 + t3611 + F::cast_from(0.27516666666666666666e-2_f64) * t5668 - F::cast_from(0.27516666666666666667e-2_f64) * t7738 + F::cast_from(0.8255e-2_f64) * t7742 - F::cast_from(0.41275e-2_f64) * t7746;
    let t7897 = -t3599 * t7757 / F::cast_from(8.0_f64) + t5687 * t2083 / F::cast_from(2.0_f64) + t1355 * t7764 / F::cast_from(4.0_f64) + t306 * t7894 / F::cast_from(2.0_f64);
    (t7878, t7879, t7894, t7897)
}
