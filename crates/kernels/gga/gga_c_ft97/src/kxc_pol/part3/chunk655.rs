//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 655/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk655<F: Float>(t1526: F, t1944: F, t7705: F, t1948: F, t342: F, t630: F, t520: F, t7773: F, t89: F, t548: F, t8078: F, t40: F, t6: F) -> (F, F, F, F, F, F) {
    let t8761 = t1526 * t7705 * t1944;
    let t8764 = t342 * t630 * t1948;
    let t8796 = t89 * t7773 * t520;
    let t8906 = t548 * t548;
    let t8907 = F::new(1.0) / t8906;
    let t8914 = F::cast_from(0.18521666970164609055e-1_f64) * t8078;
    let t8946 = t6 / t40;
    (t8761, t8764, t8796, t8907, t8914, t8946)
}
