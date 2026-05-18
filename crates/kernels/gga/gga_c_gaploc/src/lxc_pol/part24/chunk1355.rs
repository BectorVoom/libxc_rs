//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1355/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1355<F: Float>(t10496: F, t540: F, t10122: F, t1328: F, t1445: F, t1584: F, t31715: F, t31870: F, t34119: F, t34121: F, t34123: F, t34125: F, t34128: F, t34131: F, t34143: F, t34145: F, t34148: F, t34151: F, t34153: F, t34156: F, t536: F, t574: F, t597: F) -> F {
    let t34157 = t10496 * t540;
    let t34160 = -F::new(0.46011511144704899612e1) * t1584 * t1445 * t31870 + t34119 + t34121 + t34123 + t34125 + t34128 + F::new(0.71500979903700853338e0) * t536 * t34131 - F::new(0.92023022289409799224e1) * t574 * t1445 * t10122 * t1328 + F::new(0.43710935587469654631e2) * t597 * t1445 * t31715 - t34143 - t34145 - t34148 - t34151 - t34153 - t34156 + F::new(0.47667319935800568892e0) * t536 * t34157;
    t34160
}
