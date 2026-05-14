//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 749/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk749<F: Float>(t44712: F, t7290: F, t1841: F, t7289: F, t2558: F, t36390: F, t9647: F, t123: F, t36610: F, t2563: F, t35623: F, t5539: F, t42942: F, t13630: F, t2536: F, t734: F) -> (F, F, F, F, F, F, F) {
    let t44713 = t7290 * t44712;
    let t44716 = 0.17090058289204942852e-2 * t1841 * t7289 * t44713;
    let t44718 = t9647 * t36390 * t2558;
    let t44719 = 0.32043859292259267849e-3 * t44718;
    let t44720 = t36610 * t123;
    let t44722 = t9647 * t44720 * t2563;
    let t44723 = 0.96131577876777803547e-3 * t44722;
    let t44725 = t9647 * t5539 * t35623;
    let t44726 = 0.64087718584518535698e-3 * t44725;
    let t44731 = 0.1281754371690370714e-2 * t42942;
    let t44735 = 0.85450291446024714263e-3 * t1841 * t2536 * t13630 * t734;
    (t44713, t44716, t44719, t44723, t44726, t44731, t44735)
}
