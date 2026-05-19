//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 631/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk631<F: Float>(t11016: F, t3025: F, t3504: F, t5782: F, t8483: F, t935: F, t1445: F, t2087: F, t2530: F, t3009: F, t2949: F, t813: F) -> (F, F, F, F, F) {
    let t11018 = F::cast_from(0.7150097990370085334e0_f64) * t3025 * t11016;
    let t11024 = F::cast_from(0.69017266717057349418e1_f64) * t5782 * t3504;
    let t11025 = t8483 * t935;
    let t11026 = t1445 * t11025;
    let t11028 = F::cast_from(0.69017266717057349418e1_f64) * t2087 * t11026;
    let t11029 = t3009 * t2530;
    let t11030 = t1445 * t11029;
    let t11032 = F::cast_from(0.69017266717057349418e1_f64) * t2087 * t11030;
    let t11038 = t2949 * t2530;
    let t11039 = t1445 * t11038;
    let t11041 = F::cast_from(0.46011511144704899612e1_f64) * t813 * t11039;
    (t11018, t11024, t11028, t11032, t11041)
}
