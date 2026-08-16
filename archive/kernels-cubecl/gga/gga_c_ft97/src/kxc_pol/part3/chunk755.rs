//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 755/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk755<F: Float>(t15768: F, t378: F, t92: F, t15625: F, t358: F, t11167: F, t11170: F, t11172: F, t11177: F, t15734: F, t15739: F, t15744: F, t15748: F, t15750: F, t15754: F, t15758: F, t15760: F, t15765: F, t7945: F, t7946: F) -> (F, F, F, F) {
    let t15769 = t378 * t15768;
    let t15770 = t92 * t15769;
    let t15772 = t358 * t15625;
    let t15773 = t378 * t15772;
    let t15774 = t92 * t15773;
    let t15776 = -t7945 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t7946 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t11167 + t11170 - t11172 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11177 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t15734 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t15739 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t15744 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t15748 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t15750 - F::cast_from(2.0_f64) * t15754 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t15758 + t15760 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t15765 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t15770 - t15774 / F::cast_from(3.0_f64);
    (t15770, t15772, t15774, t15776)
}
