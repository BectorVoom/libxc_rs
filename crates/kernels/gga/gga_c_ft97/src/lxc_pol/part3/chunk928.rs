//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 928/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk928<F: Float>(t18302: F, t18360: F, t734: F, t91: F, t2475: F, t5120: F, t747: F, t13739: F, t13983: F, t13984: F, t13993: F, t13998: F, t14004: F, t18142: F, t18265: F, t18266: F) -> (F, F, F) {
    let t18361 = t18302 + t18360;
    let t18363 = t91 * t734 * t18361;
    let t18365 = t2475 * t5120;
    let t18367 = t91 * t18365 * t747;
    let t18369 = -F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t13739 - t13983 + t13984 - t18142 - t18265 + t18266 - t13993 + t13998 - t14004 + t18363 / F::cast_from(2.0_f64) - t18367 / F::cast_from(4.0_f64);
    (t18363, t18367, t18369)
}
