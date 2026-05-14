//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 818/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk818<F: Float>(t280: F, t8292: F, t1765: F, t362: F, t287: F, t8: F, t2320: F, t2344: F, t2433: F, t2438: F, t277: F, t4038: F, t7325: F, t7332: F, t7335: F, t7339: F, t7346: F, t7348: F, t7499: F, t7507: F, t7509: F, t7608: F, t8267: F, t8273: F, t8277: F, t8280: F, t8283: F, t8291: F, t95: F, t962: F, t984: F, t989: F) -> (F, F, F, F) {
    let t8294 = 1.0 / t280 / t8292;
    let t8297 = t1765 * t362;
    let t8298 = t8294 * t8 * t287 * t8297;
    let t8303 = -50.0 / 3.0 * t7325 * t2438 + t7332 - 4.0 * t2320 * t989 + t7335 / 2.0 - t7339 + t7346 + t7348 + 0.25844881434903430496e-2 * t95 * t277 * t8267 * t962 + 2.0 / 3.0 * t4038 * t8273 + 50.0 / 27.0 * t2433 * t8277 - t7509 - t7608 - t7499 - t7507 + 100.0 / 27.0 * t8280 * t2438 - t8283 / 3.0 + 4000000.0 / 243.0 * t8291 * t8298 + 44.0 / 3.0 * t984 * t2344;
    (t8294, t8297, t8298, t8303)
}
