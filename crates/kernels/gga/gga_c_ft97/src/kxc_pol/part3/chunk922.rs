//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 922/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk922<F: Float>(t17720: F, t17724: F, t17729: F, t17734: F, t17738: F, t17742: F, t17746: F, t17751: F, t17755: F, t17759: F, t17763: F, t13722: F, t13732: F, t17768: F, t17773: F, t17778: F, t17782: F, t17787: F, t17792: F, t17796: F, t9863: F, t9867: F) -> (F, F) {
    let t18241 = F::new(2.0) / F::new(9.0) * t17720;
    let t18252 = -t18241 + t17724 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t17729 - F::new(2.0) / F::new(9.0) * t17734 - F::new(4.0) / F::new(3.0) * t17738 - F::new(2.0) / F::new(3.0) * t17742 - F::new(2.0) * t17746 - F::new(10.0) / F::new(27.0) * t17751 + F::new(8.0) / F::new(9.0) * t17755 + F::new(2.0) / F::new(3.0) * t17759 + F::new(2.0) / F::new(9.0) * t17763;
    let t18262 = F::new(4.0) / F::new(3.0) * t17768 + t17773 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t17778 - F::new(8.0) / F::new(3.0) * t17782 - t9863 - F::new(4.0) / F::new(3.0) * t17787 - F::new(4.0) / F::new(3.0) * t17792 + F::new(4.0) / F::new(9.0) * t17796 - t9867 - F::new(8.0) / F::new(27.0) * t13722 - F::new(4.0) / F::new(9.0) * t13732;
    (t18252, t18262)
}
