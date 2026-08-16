//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 649/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk649<F: Float>(t2044: F, t7853: F, t548: F, t8078: F, t120: F, t1655: F, t40: F, t6: F, t12: F, t171: F, t341: F, t630: F) -> (F, F, F, F, F, F) {
    let t8885 = t7853 * t2044;
    let t8906 = t548 * t548;
    let t8907 = F::cast_from(1.0_f64) / t8906;
    let t8914 = F::cast_from(0.18521666970164609055e-1_f64) * t8078;
    let t8942 = t120 * t1655;
    let t8946 = t6 / t40;
    let t8947 = t12 * t171;
    let t8948 = t8946 * t8947;
    let t8959 = t341 * t630;
    (t8885, t8907, t8914, t8942, t8948, t8959)
}
