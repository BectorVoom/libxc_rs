//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 708/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk708<F: Float>(t12555: F, t12558: F, t12566: F, t12569: F, t12580: F, t13088: F, t13089: F, t13517: F, t13518: F) -> F {
    let t13520 = F::cast_from(9.0_f64) / F::cast_from(128.0_f64) * t12555;
    let t13521 = F::cast_from(9.0_f64) / F::cast_from(4096.0_f64) * t12558;
    let t13522 = F::cast_from(3.0_f64) / F::cast_from(4096.0_f64) * t12566;
    let t13523 = F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t12569;
    let t13524 = F::cast_from(4.0_f64) * t12580;
    let t13525 = t13517 + t13518 / F::cast_from(2.0_f64) + t13088 - t13089 - t13520 - t13521 + t13522 + t13523 + t13524;
    t13525
}
