//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1272/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1272<F: Float>(t18432: F, t1881: F, t2233: F, t28313: F, t28322: F, t28883: F, t28886: F, t446: F, t5407: F, t637: F, t8130: F, t8255: F, t92165: F, t92168: F, t92170: F, t92339: F, t92344: F, t92351: F, t93826: F) -> F {
    let t101823 = t8130 * t28886 / F::cast_from(8.0_f64) + t8130 * t28883 / F::cast_from(8.0_f64) - t92165 + t93826 + t92168 + t92170 + t92339 - t2233 * t18432 * t637 / F::cast_from(16.0_f64) + t1881 * t28313 / F::cast_from(8.0_f64) + t1881 * t28322 / F::cast_from(8.0_f64) + t92344 - t92351 - t446 * t5407 * t8255 / F::cast_from(8.0_f64);
    t101823
}
