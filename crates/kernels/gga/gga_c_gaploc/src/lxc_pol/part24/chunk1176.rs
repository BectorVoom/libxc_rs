//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1176/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1176<F: Float>(t10930: F, t10931: F, t32948: F, t2615: F, t326: F, t33155: F, t10847: F, t16036: F, t7630: F, t33557: F, t10851: F, t7375: F, t33627: F, t10914: F, t10915: F, t32893: F) -> (F, F, F, F, F, F, F) {
    let t33702 = 0.55213813373645879534e2 * t10930 * t10931 * t32948;
    let t33705 = 0.46011511144704899612e1 * t2615 * t326 * t33155;
    let t33708 = 0.95334639871601137784e0 * t7630 * t16036 * t10847;
    let t33711 = 0.92023022289409799224e1 * t2615 * t326 * t33557;
    let t33713 = 0.92023022289409799224e1 * t7375 * t10851;
    let t33716 = 0.92023022289409799224e1 * t2615 * t326 * t33627;
    let t33722 = 0.21450293971110256001e1 * t10914 * t10915 * t32893;
    (t33702, t33705, t33708, t33711, t33713, t33716, t33722)
}
