//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 973/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk973<F: Float>(t14657: F, t14683: F, t14655: F, t14662: F, t14666: F, t14669: F, t14673: F, t14676: F, t14680: F, t14688: F, t14692: F, t14715: F) -> (F, F) {
    let t15089 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t14657;
    let t15096 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t14683;
    let t15099 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t14655 - t15089 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14662 + t14666 / F::cast_from(9.0_f64) + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t14669 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14673 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t14676 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t14680 - t15096 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t14688 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t14692;
    let t15111 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t14715;
    (t15099, t15111)
}
