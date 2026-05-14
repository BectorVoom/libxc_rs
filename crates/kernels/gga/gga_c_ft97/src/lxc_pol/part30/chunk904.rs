//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 904/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk904<F: Float>(t224: F, t2427: F, t6789: F, t14: F, t35384: F, t173: F, t30779: F, t35409: F, t7470: F, t140943: F, t35405: F, t33445: F, t33433: F, t3766: F, t1410: F, t202: F) -> (F, F, F, F, F, F, F) {
    let t150687 = t224 * t2427 * t6789;
    let t150688 = t35384 * t14;
    let t150694 = t30779 * t7470 * t173 * t35409;
    let t150696 = t140943 * t35405;
    let t150697 = t33445 * t150696;
    let t150699 = t3766 * t33433;
    let t150704 = t202 * t1410;
    (t150687, t150688, t150694, t150696, t150697, t150699, t150704)
}
