//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1324/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1324<F: Float>(t10930: F, t10931: F, t32948: F, t2615: F, t326: F, t33155: F, t10847: F, t16036: F, t7630: F, t33557: F, t10851: F, t7375: F) -> (F, F, F, F, F) {
    let t33702 = F::cast_from(0.55213813373645879534e2_f64) * t10930 * t10931 * t32948;
    let t33705 = F::cast_from(0.46011511144704899612e1_f64) * t2615 * t326 * t33155;
    let t33708 = F::cast_from(0.95334639871601137784e0_f64) * t7630 * t16036 * t10847;
    let t33711 = F::cast_from(0.92023022289409799224e1_f64) * t2615 * t326 * t33557;
    let t33713 = F::cast_from(0.92023022289409799224e1_f64) * t7375 * t10851;
    (t33702, t33705, t33708, t33711, t33713)
}
