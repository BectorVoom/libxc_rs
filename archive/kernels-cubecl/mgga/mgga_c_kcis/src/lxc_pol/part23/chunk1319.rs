//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1319/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1319<F: Float>(t18377: F, t1881: F, t2233: F, t27328: F, t27723: F, t637: F, t8130: F, t92165: F, t92168: F, t92170: F, t92339: F, t92344: F, t92351: F, t93817: F, t97584: F) -> F {
    let t99767 = t8130 * t27328 / F::cast_from(8.0_f64) - t2233 * t18377 * t637 / F::cast_from(16.0_f64) + t93817 - t92165 + t1881 * t27723 / F::cast_from(16.0_f64) + t97584 + t92168 + t92170 + t92339 + t92344 - t92351;
    t99767
}
