//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 859/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk859<F: Float>(t103: F, t34535: F, t82: F, t23327: F, t6465: F, t34560: F, t83: F, t32597: F, t925: F, t1902: F, t34546: F, t1307: F, t452: F, t6564: F) -> (F, F, F, F, F, F, F) {
    let t34696 = t82 * t34535 * t103;
    let t34700 = t23327 * t6465;
    let t34703 = t83 * t34560;
    let t34706 = t32597 * t925;
    let t34707 = t1902 * t34706;
    let t34710 = t83 * t34546;
    let t34714 = t452 * t6564 * t1307;
    (t34696, t34700, t34703, t34706, t34707, t34710, t34714)
}
