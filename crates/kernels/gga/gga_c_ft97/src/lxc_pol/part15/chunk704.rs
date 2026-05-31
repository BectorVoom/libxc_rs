//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 704/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk704<F: Float>(t20022: F, t8314: F, t1780: F, t20031: F, t3127: F, t1787: F, t20141: F, t20334: F, t20337: F, t20341: F, t20345: F, t20349: F, t20353: F, t20356: F, t20359: F, t462: F, t92: F) -> (F, F, F, F, F) {
    let t20362 = t8314 * t20022;
    let t20363 = t1780 * t20362;
    let t20366 = t3127 * t20031;
    let t20369 = t1787 * t20141;
    let t20371 = -t92 * t20334 - t462 * t20337 / F::cast_from(3.0_f64) - F::cast_from(6.0_f64) * t92 * t20341 + F::cast_from(6.0_f64) * t462 * t20345 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t462 * t20349 - F::cast_from(2.0_f64) * t462 * t20353 + F::cast_from(2.0_f64) * t462 * t20356 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t20359 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t462 * t20363 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t20366 + t462 * t20369;
    (t20362, t20363, t20366, t20369, t20371)
}
