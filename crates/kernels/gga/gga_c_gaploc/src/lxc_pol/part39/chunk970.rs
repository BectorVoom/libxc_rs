//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 970/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk970<F: Float>(t12218: F, t1445: F, t2087: F, t2530: F, t13862: F, t2197: F, t47220: F, t833: F, t38974: F, t813: F, t935: F, t44114: F, t44118: F, t44120: F, t44124: F, t44128: F, t44131: F, t47527: F, t47531: F) -> (F,) {
    let t47535 = t2087 * t1445 * t12218 * t2530;
    let t47537 = t2197 * t13862;
    let t47540 = t833 * t1445 * t47220;
    let t47544 = t813 * t1445 * t38974 * t935;
    let t47547 = -0.44688112439813033337e-1 * t44114 - t44118 + 0.25561950635947166451e0 * t44120 + 0.42603251059911944084e-1 * t44124 - 0.69017266717057349418e1 * t47527 - 0.69017266717057349418e1 * t47531 - 0.69017266717057349418e1 * t47535 + 0.11502877786176224903e2 * t47537 + 0.11502877786176224903e2 * t47540 - 0.46011511144704899612e1 * t47544 - 0.42603251059911944084e-1 * t44128 - t44131;
    (t47547,)
}
