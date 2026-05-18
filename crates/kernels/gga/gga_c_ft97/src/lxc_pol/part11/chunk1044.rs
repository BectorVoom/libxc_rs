//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1044/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1044<F: Float>(t39370: F, t668: F, t683: F, t92: F, t41728: F, t41731: F, t41733: F, t41735: F, t41737: F, t41739: F, t41741: F, t41745: F, t41746: F, t41748: F, t41755: F) -> (F, F, F) {
    let t41757 = t668 * t39370;
    let t41759 = t92 * t683 * t41757;
    let t41761 = -F::new(2.0) / F::new(3.0) * t41728 - F::new(8.0) / F::new(9.0) * t41731 + F::new(16.0) / F::new(9.0) * t41733 - F::new(16.0) / F::new(9.0) * t41735 + F::new(8.0) / F::new(9.0) * t41737 + F::new(8.0) / F::new(3.0) * t41739 - F::new(8.0) / F::new(3.0) * t41741 + t41745 + F::new(40.0) / F::new(81.0) * t41746 + F::new(4.0) / F::new(9.0) * t41748 - F::new(80.0) / F::new(81.0) * t41755 - t41759 / F::new(3.0);
    (t41757, t41759, t41761)
}
