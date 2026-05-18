//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 987/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk987<F: Float>(t39723: F, t39708: F, t39711: F, t39715: F, t39717: F, t39721: F, t39728: F, t39732: F, t39737: F, t39741: F, t39744: F, t39747: F, t39753: F, t39757: F) -> F {
    let t40546 = F::new(8.0) / F::new(27.0) * t39723;
    let t40555 = -F::new(2.0) * t39708 + F::new(4.0) / F::new(3.0) * t39711 - F::new(2.0) / F::new(3.0) * t39715 - F::new(2.0) / F::new(9.0) * t39717 + F::new(4.0) / F::new(3.0) * t39721 + t40546 + F::new(20.0) / F::new(81.0) * t39728 + t39732 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t39737 + F::new(2.0) / F::new(9.0) * t39741 + F::new(4.0) / F::new(9.0) * t39744 - F::new(4.0) / F::new(27.0) * t39747 + F::new(4.0) / F::new(3.0) * t39753 - F::new(2.0) / F::new(3.0) * t39757;
    t40555
}
