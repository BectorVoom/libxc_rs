//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 598/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk598<F: Float>(t3784: F, t3789: F, t2660: F, t3717: F, t2767: F, t3636: F, t3641: F, t3647: F, t3653: F, t209: F, t1112: F, t3480: F) -> (F, F, F, F, F, F, F) {
    let t3790 = t3784 * t3789;
    let t3792 = t2660 * t3717;
    let t3793 = t3792 * t2767;
    let t3855 = 0.2429468532550759923e-3 * t3636 - 0.17379648562707520765e-3 * t3641 - 0.50613927761474165061e-5 * t3647 + 0.10862280351692200478e-4 * t3653;
    let t3856 = t3855 * t209;
    let t3858 = 2.0 * t3480 * t1112;
    let t3859 = t1112 * t1112;
    (t3790, t3792, t3793, t3855, t3856, t3858, t3859)
}
