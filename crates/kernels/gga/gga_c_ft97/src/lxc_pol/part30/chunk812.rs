//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 812/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk812<F: Float>(t1508: F, t6260: F, t840: F, t295: F, t312: F, t33993: F, t7679: F, t824: F, t871: F, t875: F, t2843: F, t296: F) -> (F, F, F, F, F, F, F) {
    let t34217 = t840 * t1508 * t6260;
    let t34221 = t295 * t33993 * t312;
    let t34225 = t7679 * t824;
    let t34227 = t840 * t871 * t34225;
    let t34230 = t7679 * t875;
    let t34231 = t2843 * t34230;
    let t34232 = t296 * t34231;
    (t34217, t34221, t34225, t34227, t34230, t34231, t34232)
}
