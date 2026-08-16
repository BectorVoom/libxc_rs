//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 826/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk826<F: Float>(t1017: F, t604: F, t2190: F, t12968: F, t3447: F, t8392: F, t3430: F, t9099: F, t3435: F, t2097: F, t597: F, t3441: F) -> (F, F, F, F, F) {
    let t12969 = t604 * t1017;
    let t12970 = t12969 * t2190;
    let t12971 = t12968 * t12970;
    let t12975 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8392 * t3447;
    let t12976 = t9099 * t3430;
    let t12979 = t9099 * t3435;
    let t12982 = t2097 * t597;
    let t12983 = t12982 * t3441;
    (t12971, t12975, t12976, t12979, t12983)
}
