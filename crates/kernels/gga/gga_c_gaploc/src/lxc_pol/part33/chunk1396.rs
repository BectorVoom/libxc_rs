//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1396/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1396<F: Float>(t1: F, t106: F, t12000: F, t192: F, t12075: F, t12098: F, t12124: F, t1580: F, t1599: F, t1628: F, t31018: F, t34817: F, t34821: F, t34823: F, t34827: F, t34829: F, t34831: F, t34839: F, t34842: F, t3709: F, t38281: F, t38313: F, t4598: F, t4818: F, t4820: F, t536: F, t544: F, t568: F, t574: F, t597: F, t600: F) -> F {
    let t38759 = t12000 * t1 * t106 * t192;
    let t38762 = -F::new(0.47667319935800568892e0) * t1599 * t12098 + F::new(0.61348681526273199482e1) * t1580 * t12075 + F::new(0.61348681526273199482e1) * t597 * t1628 * t12124 - F::new(0.1022478025437886658e1) * t574 * t4598 * t3709 + F::new(0.23005755572352449806e1) * t597 * t568 * t600 * t38313 + t34817 - t34821 + F::new(0.23833659967900284446e0) * t544 * t4818 * t4820 * t38281 + F::new(0.71500979903700853338e0) * t536 * t38759 + t34823 - t34827 - t34829 - t34831 - t34839 + t34842 + t31018;
    t38762
}
