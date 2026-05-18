//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1374/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1374<F: Float>(t31585: F, t4130: F, t4781: F, t590: F, t31590: F, t30572: F, t18313: F, t986: F, t31119: F, t6907: F, t10600: F, t1397: F, t1424: F) -> (F, F, F, F, F) {
    let t34431 = F::new(0.30674340763136599742e1) * t4781 * t4130 * t31585 * t590;
    let t34435 = F::new(0.30674340763136599742e1) * t4781 * t4130 * t31590 * t590;
    let t34436 = F::new(0.63904876589867916128e-1) * t30572;
    let t34439 = t18313 * t986;
    let t34441 = t31119 * t34439 * t6907;
    let t34442 = F::new(0.23005755572352449806e1) * t34441;
    let t34445 = F::new(0.79445533226334281486e-1) * t1397 * t10600 * t1424;
    (t34431, t34435, t34436, t34442, t34445)
}
