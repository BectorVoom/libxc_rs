//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 831/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk831<F: Float>(t13054: F, t2210: F, t12603: F, t144: F, t1882: F, t3567: F, t1017: F, t2180: F, t2179: F, t574: F, t1986: F, t167: F, t9432: F) -> (F, F, F, F, F) {
    let t13055 = t2210 * t13054;
    let t13058 = t144 * t12603;
    let t13062 = F::new(2.0) / F::new(9.0) * t1882 * t3567;
    let t13065 = t1017 * t2180;
    let t13067 = t574 * t2179 * t13065;
    let t13070 = t1017 * t1986;
    let t13072 = t9432 * t167 * t13070;
    (t13055, t13058, t13062, t13067, t13072)
}
