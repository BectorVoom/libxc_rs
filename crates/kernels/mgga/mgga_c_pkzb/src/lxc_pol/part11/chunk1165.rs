//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1165/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1165<F: Float>(t28658: F, t440: F, t1430: F, t3318: F, t2489: F, t8635: F, t10422: F, t1424: F, t16129: F, t82: F, t15: F, t10415: F, t10418: F, t10423: F, t10463: F, t19523: F, t2500: F, t28649: F, t28653: F, t3347: F, t34: F, t445: F, t454: F, t6723: F) -> (F, F, F, F, F, F, F) {
    let t28659 = t28658 * t440;
    let t28662 = t1430 * t3318;
    let t28665 = t2489 * t8635;
    let t28671 = t1424 * t10422 * t440;
    let t28676 = F::cast_from(6.0_f64) * t82 + F::cast_from(12.0_f64) * t16129;
    let t28677 = t15 * t28676;
    let t28684 = F::cast_from(50.0_f64) / F::cast_from(81.0_f64) * t454 * t10415 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t34 * t28649 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t19523 * t28653 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t454 * t10418 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t19523 * t28659 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t6723 * t28662 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t34 * t28665 - F::cast_from(25.0_f64) / F::cast_from(9.0_f64) * t454 * t10423 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t34 * t28671 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t34 * t28677 - F::cast_from(2200.0_f64) / F::cast_from(81.0_f64) * t10463 * t445 + F::cast_from(400.0_f64) / F::cast_from(27.0_f64) * t3347 * t2500;
    (t28659, t28662, t28665, t28671, t28676, t28677, t28684)
}
