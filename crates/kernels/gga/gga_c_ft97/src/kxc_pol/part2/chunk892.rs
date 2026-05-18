//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 892/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk892<F: Float>(t13724: F, t13761: F, t13796: F, t13825: F, t258: F, t3951: F, t761: F, t766: F, t242: F, t1175: F, t2459: F, t729: F) -> (F, F, F, F, F) {
    let t13827 = t13724 + t13761 + t13796 + t13825;
    let t13828 = t13827 * t258;
    let t13830 = t3951 * t761;
    let t13831 = t13830 * t766;
    let t13832 = t242 * t13831;
    let t13836 = t729 * t1175 * t2459;
    (t13827, t13828, t13831, t13832, t13836)
}
