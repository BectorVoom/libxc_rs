//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 540/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk540<F: Float>(t10447: F, t1445: F, t1562: F, t3354: F, t4673: F, t1572: F, t3384: F, t4950: F, t10140: F, t1457: F, t3395: F, t6985: F) -> (F, F, F, F, F, F) {
    let t10448 = t1445 * t10447;
    let t10450 = F::new(0.69017266717057349418e1) * t1562 * t10448;
    let t10455 = t4673 * t3354;
    let t10457 = F::new(0.47667319935800568892e0) * t1572 * t10455;
    let t10459 = F::new(0.71500979903700853338e0) * t4950 * t3384;
    let t10463 = t1457 * t10140;
    let t10465 = F::new(0.71500979903700853338e0) * t1572 * t10463;
    let t10466 = t6985 * t3395;
    (t10450, t10457, t10459, t10463, t10465, t10466)
}
