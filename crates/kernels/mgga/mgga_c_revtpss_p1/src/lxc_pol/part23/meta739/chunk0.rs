//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2516/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2516<F: Float>(t10760: F, t40627: F, t50613: F, t14861: F, t9794: F, t10890: F, t4458: F, t10815: F, t4426: F, t40424: F, t4430: F, t14720: F, t9775: F) -> (F, F, F, F, F, F) {
    let t51089 = t10760 * t40627 * t50613;
    let t51092 = t10760 * t9794 * t14861;
    let t51093 = F::cast_from(0.13553694749236397037e-4_f64) * t51092;
    let t51095 = t10890 * t4458;
    let t51096 = F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t51095;
    let t51098 = t10815 * t4426;
    let t51099 = F::cast_from(0.17006693853500995666e-1_f64) * t51098;
    let t51100 = t40424 * t4430;
    let t51102 = t9775 * t14720;
    (t51089, t51093, t51096, t51099, t51100, t51102)
}
