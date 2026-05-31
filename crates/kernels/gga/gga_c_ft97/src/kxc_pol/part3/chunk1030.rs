//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1030/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1030<F: Float>(t5478: F, t909: F, t4381: F, t5: F, t5429: F, t505: F, t5474: F, t1275: F, t14571: F, t16579: F, t2904: F, t333: F, t4318: F, t4322: F, t4377: F, t4382: F, t4385: F, t4635: F, t5430: F, t5475: F, t5480: F, t886: F, t889: F, t911: F, t992: F) -> F {
    let t19905 = t5478 * t909;
    let t19906 = t19905 * t4381;
    let t19920 = t5 * t5429;
    let t19927 = t5474 * t505;
    let t19939 = t889 * t19906 / F::cast_from(4.0_f64) + t4322 * t4382 / F::cast_from(2.0_f64) + t5 * t4318 * t992 / F::cast_from(2.0_f64) + t5 * t333 * t16579 / F::cast_from(4.0_f64) + t5 * t886 * t4635 / F::cast_from(4.0_f64) + t19920 * t911 / F::cast_from(4.0_f64) + t14571 * t1275 / F::cast_from(2.0_f64) + t2904 * t5475 / F::cast_from(4.0_f64) + t889 * t19927 / F::cast_from(4.0_f64) + t2904 * t5480 / F::cast_from(4.0_f64) + t4322 * t4377 / F::cast_from(2.0_f64) + t4322 * t4385 / F::cast_from(2.0_f64) + t5 * t5430 * t505 / F::cast_from(4.0_f64);
    t19939
}
