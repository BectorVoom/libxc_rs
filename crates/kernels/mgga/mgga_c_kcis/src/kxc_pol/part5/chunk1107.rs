//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1107/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1107<F: Float>(t13710: F, t13713: F, t13715: F, t13717: F, t18645: F, t18650: F, t18655: F, t18659: F, t18661: F, t18664: F, t18667: F, t18669: F, t18674: F, t18679: F, t18683: F, t9691: F, t9736: F) -> F {
    let t18685 = -t9736 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t9691 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t13710 + t13713 - t13715 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t13717 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t18645 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t18650 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t18655 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t18659 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t18661 - F::cast_from(2.0_f64) * t18664 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t18667 + t18669 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t18674 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18679 - t18683 / F::cast_from(3.0_f64);
    t18685
}
