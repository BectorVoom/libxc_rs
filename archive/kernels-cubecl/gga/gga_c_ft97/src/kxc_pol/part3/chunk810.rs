//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 810/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk810<F: Float>(t86: F, t112: F, t113: F, t1577: F, t16563: F, t16573: F, t16579: F, t3297: F, t4628: F, t4635: F, t5: F, t502: F, t505: F, t992: F) -> F {
    let t87 = F::cast_from(10000000.0_f64) <= t86;
    let t16584 = piecewise3::<F>(t87, F::cast_from(0.0_f64), t5 * t16563 * t113 / F::cast_from(4.0_f64) + t5 * t4628 * t505 / F::cast_from(4.0_f64) + t5 * t3297 * t992 / F::cast_from(2.0_f64) - t5 * t16573 * t1577 + t5 * t502 * t4635 / F::cast_from(4.0_f64) + t5 * t112 * t16579 / F::cast_from(4.0_f64));
    t16584
}
