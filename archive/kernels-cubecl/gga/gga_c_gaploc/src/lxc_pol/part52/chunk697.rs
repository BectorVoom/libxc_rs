//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 697/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk697<F: Float>(t13506: F, t7226: F, t2508: F, t12555: F, t12558: F, t12561: F, t12564: F, t12566: F, t12569: F, t471: F, t12580: F, t3603: F, t954: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13507 = t7226 * t13506;
    let t13509 = F::cast_from(0.46143157380853345701e-1_f64) * t2508 * t13507;
    let t13516 = -F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t12555 - F::cast_from(27.0_f64) / F::cast_from(4096.0_f64) * t12558 + F::cast_from(27.0_f64) / F::cast_from(262144.0_f64) * t12561 - F::cast_from(9.0_f64) / F::cast_from(262144.0_f64) * t12564 + F::cast_from(9.0_f64) / F::cast_from(4096.0_f64) * t12566 + t12569 / F::cast_from(128.0_f64);
    let t13517 = t13516 * t471;
    let t13520 = F::cast_from(9.0_f64) / F::cast_from(128.0_f64) * t12555;
    let t13521 = F::cast_from(9.0_f64) / F::cast_from(4096.0_f64) * t12558;
    let t13522 = F::cast_from(3.0_f64) / F::cast_from(4096.0_f64) * t12566;
    let t13523 = F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t12569;
    let t13524 = F::cast_from(4.0_f64) * t12580;
    let t13535 = t954 * t3603;
    (t13507, t13509, t13516, t13517, t13520, t13521, t13522, t13523, t13524, t13535)
}
