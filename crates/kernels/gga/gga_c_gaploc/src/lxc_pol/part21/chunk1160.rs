//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1160/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1160<F: Float>(t34419: F, t27071: F, t544: F, t9287: F, t10392: F, t18337: F, t31585: F, t4130: F, t4781: F, t590: F, t31590: F, t30572: F, t18313: F, t986: F, t31119: F, t6907: F) -> (F, F, F, F, F, F, F) {
    let t34420 = 0.19171462976960374838e0 * t34419;
    let t34422 = t544 * t27071 * t9287;
    let t34423 = 0.14896037479937677779e-1 * t34422;
    let t34425 = 0.30674340763136599742e1 * t18337 * t10392;
    let t34431 = 0.30674340763136599742e1 * t4781 * t4130 * t31585 * t590;
    let t34435 = 0.30674340763136599742e1 * t4781 * t4130 * t31590 * t590;
    let t34436 = 0.63904876589867916128e-1 * t30572;
    let t34439 = t18313 * t986;
    let t34441 = t31119 * t34439 * t6907;
    (t34420, t34423, t34425, t34431, t34435, t34436, t34441)
}
