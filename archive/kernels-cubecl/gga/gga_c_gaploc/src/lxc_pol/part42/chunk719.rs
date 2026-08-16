//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 719/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk719<F: Float>(t12574: F, t13088: F, t13517: F, t13520: F, t13521: F, t13522: F, t13523: F, t13524: F) -> F {
    let t14364 = t13517 + F::cast_from(2.0_f64) * t13088 - F::cast_from(2.0_f64) * t12574 - t13520 - t13521 + t13522 + t13523 + t13524;
    t14364
}
