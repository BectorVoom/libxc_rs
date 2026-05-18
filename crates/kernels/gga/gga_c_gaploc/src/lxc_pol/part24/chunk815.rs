//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 815/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk815<F: Float>(t2612: F, t5676: F, t2611: F, t549: F, t2033: F, t1391: F, t2723: F, t825: F, t2013: F, t2607: F, t6574: F, t823: F) -> (F, F, F, F, F) {
    let t7792 = t5676 * t2612;
    let t7794 = t549 * t2611;
    let t7795 = t2033 * t7794;
    let t7797 = t1391 * t2723;
    let t7798 = t825 * t7797;
    let t7800 = t2013 * t2607;
    let t7802 = t823 * t6574;
    (t7792, t7795, t7798, t7800, t7802)
}
