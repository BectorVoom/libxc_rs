//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 348/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk348<F: Float>(t1524: F, t493: F, t124: F, t4: F, t495: F, t128: F, t511: F, t8: F, t134: F, t122: F, t186: F, t21: F) -> (F, F, F, F, F, F) {
    let t1525 = t1524 * t493;
    let t1532 = t495 * t124 * t4;
    let t1535 = t1524 * t128;
    let t1539 = F::new(1.0) / t8 / t511;
    let t1540 = t1539 * t134;
    let t1543 = F::new(1.0) / t186 / t122;
    let t1545 = t1543 * t124 * t21;
    (t1525, t1532, t1535, t1539, t1540, t1545)
}
