//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 851/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk851<F: Float>(t13710: F, t13713: F, t13715: F, t13717: F, t18645: F, t18650: F, t18655: F, t18659: F, t18661: F, t18664: F, t18667: F, t18669: F, t18674: F, t18679: F, t18683: F, t9691: F, t9736: F) -> F {
    let t18685 = -t9736 - F::new(4.0) / F::new(27.0) * t9691 - F::new(8.0) / F::new(27.0) * t13710 + t13713 - t13715 + F::new(4.0) / F::new(9.0) * t13717 + F::new(2.0) / F::new(27.0) * t18645 - F::new(10.0) / F::new(27.0) * t18650 + F::new(4.0) / F::new(3.0) * t18655 - F::new(8.0) / F::new(9.0) * t18659 - F::new(2.0) / F::new(9.0) * t18661 - F::new(2.0) * t18664 + F::new(8.0) / F::new(3.0) * t18667 + t18669 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t18674 + F::new(2.0) / F::new(3.0) * t18679 - t18683 / F::new(3.0);
    t18685
}
