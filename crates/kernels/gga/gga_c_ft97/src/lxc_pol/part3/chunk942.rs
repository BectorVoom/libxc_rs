//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 942/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk942<F: Float>(t13722: F, t13732: F, t14317: F, t14318: F, t17768: F, t17773: F, t17778: F, t17782: F, t17787: F, t17792: F, t17796: F, t13739: F, t13747: F, t13754: F, t13781: F, t13795: F, t13810: F, t18142: F, t18145: F, t18148: F, t18363: F, t18367: F) -> (F, F) {
    let t18567 = F::new(4.0) / F::new(9.0) * t17768 + t17773 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t17778 - F::new(8.0) / F::new(9.0) * t17782 - t14317 - F::new(4.0) / F::new(9.0) * t17787 - F::new(4.0) / F::new(9.0) * t17792 + F::new(4.0) / F::new(27.0) * t17796 - t14318 - F::new(8.0) / F::new(81.0) * t13722 - F::new(4.0) / F::new(27.0) * t13732;
    let t18575 = -F::new(8.0) / F::new(27.0) * t13739 - t13747 + t13754 - t18142 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t18145 + t18148 / F::new(9.0) - t13781 + t13795 - t13810 + t18363 / F::new(6.0) - t18367 / F::new(12.0);
    (t18567, t18575)
}
