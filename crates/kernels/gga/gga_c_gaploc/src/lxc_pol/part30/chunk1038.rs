//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1038/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1038<F: Float>(t4324: F, t9439: F, t1428: F, t4461: F, t103: F, t23: F, t417: F, t8: F, t1210: F, t62: F, t123: F, t1559: F) -> (F, F, F, F, F, F) {
    let t18823 = t9439 * t4324;
    let t18970 = t4461 * t1428;
    let t19077 = t23 * t103;
    let t19223 = t8 * t417;
    let t19244 = t62 * t1210;
    let t19531 = t1559 * t123;
    (t18823, t18970, t19077, t19223, t19244, t19531)
}
