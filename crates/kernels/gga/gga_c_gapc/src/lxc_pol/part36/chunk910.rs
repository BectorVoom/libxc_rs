//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 910/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk910<F: Float>(t10284: F, t10290: F, t10295: F, t10299: F, t10303: F, t10306: F, t10311: F, t10314: F, t10317: F, t10319: F, t10321: F, t10326: F, t10330: F, t10333: F, t10337: F, t10341: F, t10344: F, t10351: F, t10358: F, t10361: F, t10364: F, t10368: F) -> (F, F) {
    let t11123 = -F::new(0.41758041133049637282e-5) * t10284 + F::new(0.11636624900248636096e-6) * t10290 + F::new(0.685007236434541294e-5) * t10295 + F::new(0.41758041133049637282e-5) * t10299 + F::new(0.22833574547818043134e-6) * t10303 + F::new(0.3757753982726626527e-4) * t10306 + F::new(0.66812865812879419652e-4) * t10311 + F::new(0.23485962392041415794e-5) * t10314 + F::new(0.16414765573575218917e-4) * t10317 + F::new(0.66812865812879419652e-4) * t10319 - F::new(0.15589668689671864586e-3) * t10321;
    let t11136 = F::new(0.23485962392041415794e-4) * t10326 + F::new(0.685007236434541294e-5) * t10330 - F::new(0.23485962392041415794e-4) * t10333 + F::new(0.79793999315990647512e-6) * t10337 + F::new(0.16414765573575218917e-4) * t10341 - F::new(0.47876399589594388508e-5) * t10344 - F::new(0.79793999315990647512e-6) * t10351 - F::new(0.79793999315990647512e-6) * t10358 - F::new(0.56366309740899397906e-3) * t10361 - F::new(0.11273261948179879581e-2) * t10364 + F::new(0.54715885245250729722e-5) * t10368;
    (t11123, t11136)
}
