//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1307/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1307<F: Float>(t2679: F, t8802: F, t9800: F, t10942: F, t1966: F, t11119: F, t1986: F, t1445: F, t2087: F, t2530: F, t8483: F, t3009: F, t7112: F) -> (F, F, F, F, F) {
    let t33492 = t9800 * t8802 * t2679;
    let t33493 = F::cast_from(0.19171462976960374838e1_f64) * t33492;
    let t33494 = t1966 * t10942;
    let t33495 = F::cast_from(0.25561950635947166451e1_f64) * t33494;
    let t33496 = t1986 * t11119;
    let t33497 = F::cast_from(0.51123901271894332902e0_f64) * t33496;
    let t33501 = F::cast_from(0.13803453343411469884e2_f64) * t2087 * t1445 * t8483 * t2530;
    let t33505 = F::cast_from(0.69017266717057349418e1_f64) * t2087 * t1445 * t3009 * t7112;
    (t33493, t33495, t33497, t33501, t33505)
}
