//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1582/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1582<F: Float>(t14613: F, t14657: F, t1055: F, t10160: F, t10170: F, t1052: F, t1066: F, t11010: F, t14545: F, t14549: F, t14552: F, t14555: F, t14562: F, t1635: F, t3169: F, t3176: F, t3207: F, t388: F, t4557: F, t4660: F, t4665: F) -> (F, F, F) {
    let t14658 = t14613 + t14657;
    let t14659 = t1055 * t14658;
    let t14661 = -F::cast_from(2.0_f64) * t10160 * t1635 - t10170 * t1635 + F::cast_from(2.0_f64) * t1052 * t14549 - t1052 * t14659 - F::cast_from(2.0_f64) * t1066 * t14545 - F::cast_from(2.0_f64) * t1066 * t14552 - F::cast_from(2.0_f64) * t1066 * t14555 - t11010 * t1635 + F::cast_from(2.0_f64) * t14562 * t388 + F::cast_from(4.0_f64) * t3169 * t4665 + F::cast_from(2.0_f64) * t3176 * t4557 + F::cast_from(2.0_f64) * t3176 * t4660 - t3207 * t4557;
    (t14658, t14659, t14661)
}
