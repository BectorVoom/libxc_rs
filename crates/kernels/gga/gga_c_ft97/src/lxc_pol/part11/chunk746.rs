//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 746/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk746<F: Float>(t2610: F, t8392: F, t2526: F, t761: F, t684: F, t2606: F, t9520: F, t9695: F, t9701: F, t9705: F, t9711: F, t9715: F, t9720: F, t9723: F, t9727: F, t9730: F, t9735: F, t9739: F, t9752: F) -> (F, F, F, F, F) {
    let t10090 = t8392 * t2610;
    let t10092 = t761 * t2526;
    let t10093 = t10092 * t684;
    let t10094 = t2606 * t10093;
    let t10108 = -t9705 / F::new(3.0) + F::new(6.0) * t9715 - F::new(10.0) / F::new(27.0) * t9720 + t9723 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t9727 - F::new(4.0) / F::new(9.0) * t9735 - F::new(2.0) * t9739 + F::new(4.0) / F::new(3.0) * t9752 - t9695 - F::new(4.0) / F::new(3.0) * t9701 - F::new(6.0) * t9711 - F::new(2.0) * t9730 + t9520;
    (t10090, t10092, t10093, t10094, t10108)
}
