//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1105/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1105<F: Float>(t32174: F, t34872: F, t173: F, t34871: F, t7195: F, t23839: F, t26643: F, t32233: F, t3379: F, t420: F, t71: F, t145074: F, t23711: F) -> (F, F, F, F, F, F) {
    let t147274 = t32174 * t34872;
    let t147278 = t7195 * t173 * t34871;
    let t147279 = t23839 * t147278;
    let t147291 = t32233 * t26643;
    let t147298 = t7195 * t420 * t71 * t3379;
    let t147308 = t23711 * t145074;
    (t147274, t147278, t147279, t147291, t147298, t147308)
}
