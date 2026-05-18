//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 833/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk833<F: Float>(t44725: F, t42942: F, t13630: F, t1841: F, t2536: F, t734: F, t42953: F, t2576: F, t35435: F, t161: F, t36610: F, t42963: F) -> (F, F, F, F, F, F, F) {
    let t44726 = F::new(0.64087718584518535698e-3) * t44725;
    let t44731 = F::new(0.1281754371690370714e-2) * t42942;
    let t44735 = F::new(0.85450291446024714263e-3) * t1841 * t2536 * t13630 * t734;
    let t44740 = F::new(0.17090058289204942853e-2) * t42953;
    let t44744 = F::new(0.59815204012217299984e-2) * t1841 * t35435 * t2576;
    let t44745 = t36610 * t161;
    let t44748 = F::new(0.25635087433807414279e-2) * t1841 * t44745 * t2576;
    let t44751 = F::new(0.15381052460284448568e-1) * t42963;
    (t44726, t44731, t44735, t44740, t44744, t44748, t44751)
}
