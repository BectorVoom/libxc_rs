//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 375/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk375<F: Float>(t215: F, t220: F, t1228: F, t286: F, t708: F, t284: F, t712: F) -> (F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t1653 = t215 * t215;
    let t1654 = F::cast_from(1.0_f64) / t1653;
    let t1665 = t220 * t220;
    let t1666 = F::cast_from(1.0_f64) / t1665;
    let t1681 = t1228 * t286 * t708;
    let t1683 = t284 * t284;
    let t1685 = F::cast_from(1.0_f64) / t1683 / t284;
    let t1687 = t1685 * pi * t712;
    (t1654, t1666, t1681, t1683, t1685, t1687)
}
