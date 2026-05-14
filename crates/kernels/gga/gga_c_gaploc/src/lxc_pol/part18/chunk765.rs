//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 765/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk765<F: Float>(t325: F, t883: F, t900: F, t6117: F, t1710: F, t2610: F, t2365: F, t2033: F, t2612: F, t5676: F, t2611: F, t549: F, t1391: F, t2723: F, t825: F, t2013: F, t2607: F) -> (F, F, F, F, F, F) {
    let t7784 = t883 * t325;
    let t7785 = t900 * t7784;
    let t7786 = t6117 * t7785;
    let t7788 = t2610 * t1710;
    let t7789 = t2365 * t7788;
    let t7790 = t2033 * t7789;
    let t7792 = t5676 * t2612;
    let t7794 = t549 * t2611;
    let t7795 = t2033 * t7794;
    let t7797 = t1391 * t2723;
    let t7798 = t825 * t7797;
    let t7800 = t2013 * t2607;
    (t7786, t7790, t7792, t7795, t7798, t7800)
}
