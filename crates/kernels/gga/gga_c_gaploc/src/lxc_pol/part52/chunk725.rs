//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 725/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk725<F: Float>(t14384: F, t2580: F, t14377: F, t738: F, t13488: F, t13490: F, t13494: F, t13497: F, t13501: F, t13504: F, t13509: F, t13935: F, t13938: F, t2508: F, t270: F) -> (F, F, F) {
    let t14415 = t2580 * t14384;
    let t14420 = t738 * t14377;
    let t14425 = F::new(0.30762104920568897134e-1) * t2508 * t14415 - F::new(0.1281754371690370714e-2) * t13935 - t13488 - F::new(0.96131577876777803547e-3) * t13490 + t13494 + t13497 + t13501 - F::new(0.76905262301422242837e-2) * t270 * t14420 + F::new(0.64087718584518535698e-3) * t13504 - t13509 + F::new(0.1281754371690370714e-2) * t13938;
    (t14415, t14420, t14425)
}
