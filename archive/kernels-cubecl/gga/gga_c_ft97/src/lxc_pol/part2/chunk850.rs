//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 850/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk850<F: Float>(t147: F, t13254: F, t13289: F, t3746: F, t713: F, t2493: F, t1934: F, t3699: F) -> (F, F, F, F) {
    let t148 = F::cast_from(10000000.0_f64) <= t147;
    let t13291 = piecewise3::<F>(t148, F::cast_from(0.0_f64), t13254 + t13289);
    let t13292 = t3746 * t713;
    let t13293 = t2493 * t13292;
    let t13296 = t3699 * t1934;
    (t13291, t13292, t13293, t13296)
}
