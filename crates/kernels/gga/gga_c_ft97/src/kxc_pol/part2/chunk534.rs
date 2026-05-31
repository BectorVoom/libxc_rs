//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 534/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk534<F: Float>(t86: F, t112: F, t18: F, t113: F, t1577: F, t3297: F, t5: F, t502: F, t505: F, t989: F, t992: F, t1022: F, t1952: F) -> (F, F, F) {
    let t87 = F::cast_from(10000000.0_f64) <= t86;
    let t3307 = t112 * t18;
    let t3312 = piecewise3::<F>(t87, F::cast_from(0.0_f64), t5 * t3297 * t113 / F::cast_from(4.0_f64) + t5 * t989 * t505 / F::cast_from(4.0_f64) + t5 * t502 * t992 / F::cast_from(4.0_f64) - t5 * t3307 * t1577 / F::cast_from(2.0_f64));
    let t3313 = t1952 * t1022;
    (t3307, t3312, t3313)
}
