//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1166/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1166<F: Float>(t31555: F, t10253: F, t2312: F, t21154: F, t2268: F, t25775: F, t10160: F, t1349: F, t25730: F, t4261: F, t9074: F, t10276: F, t3808: F) -> (F, F, F, F, F, F) {
    let t31556 = F::cast_from(0.47425011059460249332e-2_f64) * t31555;
    let t31564 = t2312 * t10253;
    let t31565 = F::cast_from(0.23712505529730124666e-2_f64) * t31564;
    let t31568 = F::cast_from(0.17073003981405689759e1_f64) * t2268 * t25775 * t21154;
    let t31569 = t1349 * t10160;
    let t31570 = F::cast_from(0.31616674039640166222e-2_f64) * t31569;
    let t31574 = t9074 * t4261 * t25730;
    let t31575 = F::cast_from(0.47425011059460249332e-2_f64) * t31574;
    let t31577 = F::cast_from(0.18970004423784099733e-1_f64) * t3808 * t10276;
    (t31556, t31565, t31568, t31570, t31575, t31577)
}
