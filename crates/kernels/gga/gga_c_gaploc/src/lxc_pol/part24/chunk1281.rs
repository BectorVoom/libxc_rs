//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1281/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1281<F: Float>(t326: F, t32948: F, t825: F, t11109: F, t5840: F, t10856: F, t2033: F, t549: F, t10811: F, t7751: F, t32893: F, t10906: F, t2013: F) -> (F, F, F, F, F, F) {
    let t33067 = F::new(0.18404604457881959845e2) * t825 * t326 * t32948;
    let t33068 = t5840 * t11109;
    let t33069 = F::new(0.51123901271894332902e0) * t33068;
    let t33071 = t2033 * t549 * t10856;
    let t33072 = F::new(0.59584149919750711116e-1) * t33071;
    let t33074 = F::new(0.42900587942220512003e1) * t10811 * t7751;
    let t33077 = F::new(0.92023022289409799224e1) * t825 * t326 * t32893;
    let t33079 = F::new(0.18404604457881959845e2) * t2013 * t10906;
    (t33067, t33069, t33072, t33074, t33077, t33079)
}
