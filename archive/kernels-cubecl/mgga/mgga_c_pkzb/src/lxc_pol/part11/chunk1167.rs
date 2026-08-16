//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1167/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1167<F: Float>(t27: F, t28676: F, t19530: F, t23870: F, t2504: F, t28696: F, t28700: F, t28704: F, t28707: F, t28710: F, t28714: F, t3347: F, t38: F, t6738: F, t8646: F, t8650: F, t8654: F, t8658: F, t991: F) -> (F, F) {
    let t28718 = -t27 * t28676;
    let t28721 = -F::cast_from(200.0_f64) / F::cast_from(9.0_f64) * t3347 * t2504 + F::cast_from(50.0_f64) / F::cast_from(27.0_f64) * t991 * t8646 + F::cast_from(100.0_f64) / F::cast_from(9.0_f64) * t23870 * t8650 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t991 * t8654 - F::cast_from(25.0_f64) / F::cast_from(3.0_f64) * t991 * t8658 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t38 * t28696 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t19530 * t28700 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t19530 * t28704 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t6738 * t28707 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t38 * t28710 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t38 * t28714 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t38 * t28718;
    (t28718, t28721)
}
