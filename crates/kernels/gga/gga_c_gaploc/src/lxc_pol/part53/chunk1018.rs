//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1018/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1018<F: Float>(t41729: F, t41734: F, t41735: F, t41736: F, t41737: F, t41741: F, t41743: F, t41744: F, t41747: F, t41752: F, t41753: F, t41754: F, t41759: F, t41761: F, t41767: F, t41773: F, t41777: F, t41781: F, t47892: F, t47895: F) -> F {
    let t50843 = -t41729 - t41734 - t41735 - t41736 + t41737 + F::new(0.59584149919750711116e-1) * t47892 + t41741 + t41743 - t41744 + t41747 - t41752 - t41753 + t41754 - t47895 - t41759 + t41761 + t41767 - t41773 + t41777 + t41781;
    t50843
}
