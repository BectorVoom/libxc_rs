//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 865/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk865<F: Float>(t12889: F, t12890: F, t16668: F, t16673: F, t16677: F, t16679: F, t16684: F, t16689: F, t16692: F, t16696: F, t16699: F, t12343: F, t12346: F, t12359: F, t12362: F, t12571: F, t12891: F, t12897: F, t12911: F, t12914: F, t16706: F, t9383: F) -> (F, F) {
    let t17454 = -F::new(4.0) / F::new(9.0) * t16668 - F::new(4.0) / F::new(9.0) * t16673 + F::new(4.0) / F::new(27.0) * t16677 - F::new(2.0) / F::new(27.0) * t16679 + t16684 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t16689 + F::new(8.0) / F::new(9.0) * t16692 + t16696 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t16699 - t12889 - t12890;
    let t17459 = t12891 - t12897 - t12343 - t12346 - t12911 + F::new(4.0) / F::new(27.0) * t12359 - F::new(8.0) / F::new(81.0) * t12362 - t9383 + t12914 - F::new(8.0) / F::new(27.0) * t12571 - F::new(2.0) / F::new(27.0) * t16706;
    (t17454, t17459)
}
