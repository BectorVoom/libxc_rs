//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 781/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk781<F: Float>(t11490: F, t32606: F, t23327: F, t5631: F, t32417: F, t83: F, t1825: F, t452: F, t7229: F, t1307: F, t5743: F, t488: F) -> (F, F, F, F, F, F) {
    let t32607 = t11490 * t32606;
    let t32610 = t23327 * t5631;
    let t32613 = t83 * t32417;
    let t32617 = t452 * t1825 * t7229;
    let t32620 = t1307 * t5743;
    let t32622 = t452 * t488 * t32620;
    (t32607, t32610, t32613, t32617, t32620, t32622)
}
