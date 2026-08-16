//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2398/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2398<F: Float>(t41904: F, t47787: F, t59663: F, t59665: F, t59680: F, t59688: F, t59694: F, t59700: F, t59702: F, t59704: F, t59759: F, t59761: F, t68586: F, t68589: F, t68592: F, t68596: F, t68599: F, t68602: F, t68605: F, t68608: F) -> F {
    let t68693 = F::cast_from(2.0_f64) * t68586 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t68589 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t68592 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t68596 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t68599 + F::cast_from(4.0_f64) * t68602 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t68605 - F::cast_from(6.0_f64) * t68608 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t59663 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t59665 + t59680 / F::cast_from(3.0_f64) + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t59688 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t59694 + t41904 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t59700 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t59702 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t59704 + F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t47787 + F::cast_from(2.0_f64) * t59759 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t59761;
    t68693
}
