//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 537/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk537<F: Float>(t2987: F, t984: F, t343: F, t883: F, t607: F, t2822: F, t225: F, t991: F, t1008: F, t191: F) -> (F, F, F, F, F, F) {
    let t2988 = t2987 * t984;
    let t2989 = t343 * t883;
    let t2990 = t2989 * t607;
    let t3003 = F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t2822;
    let t3026 = t991 * t225;
    let t3030 = F::cast_from(1.0_f64) / t1008 / t191;
    (t2988, t2989, t2990, t3003, t3026, t3030)
}
