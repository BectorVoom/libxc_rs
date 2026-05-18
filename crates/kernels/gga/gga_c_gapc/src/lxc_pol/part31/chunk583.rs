//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 583/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk583<F: Float>(t2701: F, t646: F, t918: F, t3343: F, t1026: F, t933: F, t937: F, t1081: F, t954: F, t3314: F, t3316: F, t3318: F, t3323: F, t3331: F, t3334: F, t3338: F, t3341: F) -> (F, F, F, F, F, F) {
    let t3345 = t646 * t918 * t2701;
    let t3346 = t3343 * t3345;
    let t3348 = t933 * t1026;
    let t3349 = t3348 * t937;
    let t3351 = t1081 * t954;
    let t3353 = F::new(0.50602213541666666669e-5) * t3314 - F::new(0.25301106770833333334e-4) * t3316 - F::new(0.3243554543208642639e-2) * t3318 - F::new(0.25013570439533790734e-8) * t3323 - F::new(0.12309827972211511188e-7) * t3331 + F::new(0.10567613244746075633e-6) * t3334 + F::new(0.86880925264517213544e-4) * t3338 + F::new(0.86880925264517213544e-4) * t3341 - F::new(0.12872857093359300474e-5) * t3346 + F::new(0.11594181388521408695e-4) * t3349 - F::new(0.2318836277704281739e-4) * t3351;
    (t3345, t3346, t3348, t3349, t3351, t3353)
}
