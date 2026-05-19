//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 388/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk388<F: Float>(t554: F, t72: F, t5579: F, t1355: F, t5608: F, t3392: F, t5812: F) -> (F, F, F, F) {
    let t5830 = t72 * t554;
    let t5831 = t5579 * t5830;
    let t5837 = F::cast_from(0.16669500273148148149e-1_f64) * t1355 * t5608;
    let t5838 = t3392 * t5812;
    (t5830, t5831, t5837, t5838)
}
