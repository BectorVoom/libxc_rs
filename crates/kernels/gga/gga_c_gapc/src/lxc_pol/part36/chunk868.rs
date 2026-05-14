//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 868/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk868<F: Float>(t11840: F, t3304: F, t3285: F, t3775: F, t3289: F, t19: F, t825: F, t147: F, t311: F, t11579: F, t919: F, t128: F, t2211: F, t2545: F, t2578: F, t3297: F, t3761: F, t869: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11841 = t11840 * t3304;
    let t11843 = t3775 * t3285;
    let t11845 = t3775 * t3289;
    let t11847 = t825 * t19;
    let t11848 = t11847 * t147;
    let t11849 = t311 * t11848;
    let t11850 = t11579 * t919;
    let t11851 = t11849 * t11850;
    let t11853 = t2211 * t128;
    let t11854 = t2545 * t11853;
    let t11855 = t2578 * t11854;
    let t11858 = t869 * t3761 * t3297;
    (t11841, t11843, t11845, t11848, t11849, t11850, t11851, t11853, t11854, t11855, t11858)
}
