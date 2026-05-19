//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 581/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk581<F: Float>(t2701: F, t646: F, t918: F, t3343: F, t1026: F, t933: F, t937: F, t1081: F, t954: F, t3314: F, t3316: F, t3318: F, t3323: F, t3331: F, t3334: F, t3338: F, t3341: F) -> (F, F, F, F, F, F) {
    let t3345 = t646 * t918 * t2701;
    let t3346 = t3343 * t3345;
    let t3348 = t933 * t1026;
    let t3349 = t3348 * t937;
    let t3351 = t1081 * t954;
    let t3353 = F::cast_from(0.50602213541666666669e-5_f64) * t3314 - F::cast_from(0.25301106770833333334e-4_f64) * t3316 - F::cast_from(0.3243554543208642639e-2_f64) * t3318 - F::cast_from(0.25013570439533790734e-8_f64) * t3323 - F::cast_from(0.12309827972211511188e-7_f64) * t3331 + F::cast_from(0.10567613244746075633e-6_f64) * t3334 + F::cast_from(0.86880925264517213544e-4_f64) * t3338 + F::cast_from(0.86880925264517213544e-4_f64) * t3341 - F::cast_from(0.12872857093359300474e-5_f64) * t3346 + F::cast_from(0.11594181388521408695e-4_f64) * t3349 - F::cast_from(0.2318836277704281739e-4_f64) * t3351;
    (t3345, t3346, t3348, t3349, t3351, t3353)
}
