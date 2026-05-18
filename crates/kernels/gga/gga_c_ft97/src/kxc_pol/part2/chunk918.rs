//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 918/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk918<F: Float>(t1882: F, t3983: F, t1131: F, t2459: F, t2574: F, t265: F, t3746: F, t724: F, t773: F, t3839: F, t1140: F, t8232: F) -> (F, F, F, F, F) {
    let t14212 = F::new(2.0) / F::new(9.0) * t1882 * t3983;
    let t14213 = t1131 * t2459;
    let t14215 = t2574 * t265 * t14213;
    let t14219 = t724 * t773 * t3746;
    let t14223 = F::new(4.0) / F::new(9.0) * t1882 * t3839;
    let t14224 = t8232 * t1140;
    (t14212, t14215, t14219, t14223, t14224)
}
