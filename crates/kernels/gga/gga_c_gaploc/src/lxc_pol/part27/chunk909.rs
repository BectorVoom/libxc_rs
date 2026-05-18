//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 909/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk909<F: Float>(t2326: F, t900: F, t9561: F, t1407: F, t3178: F, t3163: F, t4379: F, t2293: F, t2366: F, t2365: F, t1429: F, t6696: F, t901: F) -> (F, F, F, F, F, F, F, F) {
    let t9562 = t900 * t2326;
    let t9564 = F::new(0.89376224879626066674e-1) * t9561 * t9562;
    let t9568 = t1407 * t3178;
    let t9571 = F::new(0.29792074959875355558e-1) * t4379 * t3163;
    let t9572 = t2366 * t2293;
    let t9573 = t2365 * t9572;
    let t9575 = F::new(0.29792074959875355558e-1) * t1429 * t9573;
    let t9577 = F::new(0.29792074959875355558e-1) * t6696 * t901;
    (t9562, t9564, t9568, t9571, t9572, t9573, t9575, t9577)
}
