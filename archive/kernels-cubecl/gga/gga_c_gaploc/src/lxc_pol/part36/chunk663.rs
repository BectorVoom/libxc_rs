//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 663/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk663<F: Float>(t11016: F, t3025: F, t10782: F, t701: F, t1445: F, t3504: F, t5782: F, t8483: F, t935: F, t2087: F, t2530: F, t3009: F) -> (F, F, F, F, F) {
    let t11018 = F::cast_from(0.7150097990370085334e0_f64) * t3025 * t11016;
    let t11019 = t10782 * t701;
    let t11020 = t1445 * t11019;
    let t11024 = F::cast_from(0.69017266717057349418e1_f64) * t5782 * t3504;
    let t11025 = t8483 * t935;
    let t11026 = t1445 * t11025;
    let t11028 = F::cast_from(0.69017266717057349418e1_f64) * t2087 * t11026;
    let t11029 = t3009 * t2530;
    (t11018, t11020, t11024, t11028, t11029)
}
