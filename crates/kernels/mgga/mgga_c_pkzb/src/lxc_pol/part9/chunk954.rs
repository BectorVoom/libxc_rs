//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 954/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk954<F: Float>(t7431: F, t7442: F, t684: F, t664: F, t2793: F, t694: F, t7335: F, t5522: F, t7332: F, t7352: F, t7361: F, t7363: F, t7366: F, t7368: F, t7371: F, t7373: F, t7376: F, t7379: F) -> (F, F, F, F, F) {
    let t7443 = t7431 + t7442;
    let t7444 = t7443 * t684;
    let t7446 = F::cast_from(1.0_f64) * t664 * t7444;
    let t7447 = t2793 * t694;
    let t7451 = F::cast_from(0.60385e0_f64) * t7335;
    let t7462 = F::cast_from(0.27595e0_f64) * t7332 - t7451 + F::cast_from(0.905775e0_f64) * t7352 + F::cast_from(0.16504875e0_f64) * t7361 + F::cast_from(0.258925e1_f64) * t7363 - F::cast_from(0.258925e1_f64) * t7366 - F::cast_from(0.1294625e1_f64) * t7368 + F::cast_from(0.16504875e0_f64) * t7371 + F::cast_from(0.82524375e-1_f64) * t7373 + F::cast_from(0.19419375e1_f64) * t7376 - F::cast_from(0.412621875e-1_f64) * t7379 + F::cast_from(0.80513333333333333334e0_f64) * t5522;
    (t7443, t7444, t7446, t7447, t7462)
}
