//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 594/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk594<F: Float>(t376: F, t6422: F, t5743: F, t979: F, t1852: F, t10969: F, t5731: F, t1332: F, t3255: F, t492: F, t6557: F, t379: F, t6421: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25587 = t376 * t6422;
    let t25590 = t5743 * t979;
    let t25591 = t1852 * t25590;
    let t25593 = t10969 * t5731;
    let t25595 = t1332 * t3255;
    let t25596 = t1852 * t25595;
    let t25598 = t6557 * t492;
    let t25599 = t1852 * t25598;
    let t25601 = t6421 * t379;
    (t25587, t25590, t25591, t25593, t25595, t25596, t25598, t25599, t25601)
}
