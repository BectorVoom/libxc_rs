//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1143/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1143<F: Float>(t2464: F, t587: F, t9444: F, t2487: F, t9449: F, t7014: F, t9368: F, t2488: F, t30258: F, t1391: F, t9367: F, t4391: F, t549: F, t6510: F) -> (F, F, F, F, F, F) {
    let t30762 = t587 * t2464 * t9444;
    let t30765 = t2487 * t2464 * t9449;
    let t30768 = F::cast_from(0.17041300423964777634e0_f64) * t7014 * t9368;
    let t30770 = t2487 * t2488 * t30258;
    let t30773 = t2487 * t1391 * t9367;
    let t30778 = F::cast_from(0.23833659967900284446e0_f64) * t4391 * t549 * t6510;
    (t30762, t30765, t30768, t30770, t30773, t30778)
}
