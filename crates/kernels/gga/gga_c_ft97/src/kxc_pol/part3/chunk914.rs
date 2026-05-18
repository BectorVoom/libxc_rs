//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 914/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk914<F: Float>(t18117: F, t92: F, t17727: F, t683: F, t16579: F, t668: F, t13538: F, t13541: F, t13543: F, t13544: F, t18096: F, t18099: F, t18102: F, t18105: F, t18107: F, t18110: F, t18113: F, t18115: F, t9557: F, t9558: F) -> (F, F, F, F, F) {
    let t18118 = t92 * t18117;
    let t18120 = t683 * t17727;
    let t18121 = t92 * t18120;
    let t18123 = t668 * t16579;
    let t18124 = t683 * t18123;
    let t18125 = t92 * t18124;
    let t18127 = -t9557 - F::new(4.0) / F::new(27.0) * t9558 - F::new(8.0) / F::new(27.0) * t13538 + t13541 - t13543 - F::new(4.0) / F::new(9.0) * t13544 + F::new(2.0) / F::new(27.0) * t18096 - F::new(10.0) / F::new(27.0) * t18099 + F::new(4.0) / F::new(3.0) * t18102 + F::new(8.0) / F::new(9.0) * t18105 - F::new(2.0) / F::new(9.0) * t18107 - F::new(2.0) * t18110 - F::new(8.0) / F::new(3.0) * t18113 + t18115 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t18118 + F::new(2.0) / F::new(3.0) * t18121 - t18125 / F::new(3.0);
    (t18118, t18121, t18123, t18125, t18127)
}
