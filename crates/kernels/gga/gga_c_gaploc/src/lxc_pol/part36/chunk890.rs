//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 890/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk890<F: Float>(t41647: F, t41707: F, t41748: F, t41806: F, t41855: F, t41911: F, t41964: F, t41997: F, t42060: F, t42142: F, t42185: F, t42246: F, t42306: F, t42360: F, t42418: F, t42462: F, t502: F) -> F {
    let t42467 = t502 * (t41647 + t41707 + t41748 + t41806 + t41855 + t41911 + t41964 + t41997 + t42060 + t42142 + t42185 + t42246 + t42306 + t42360 + t42418 + t42462);
    t42467
}
