//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 832/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk832<F: Float>(t44318: F, t35893: F, t4261: F, t9074: F, t11280: F, t2326: F, t2268: F, t2440: F, t3518: F, t44268: F, t447: F, t13319: F, t6313: F) -> (F, F, F, F, F, F) {
    let t44319 = F::new(0.35568758294595186999e-2) * t44318;
    let t44321 = t9074 * t4261 * t35893;
    let t44322 = F::new(0.23712505529730124666e-2) * t44321;
    let t44324 = t9074 * t11280 * t2326;
    let t44325 = F::new(0.82993769354055436331e-2) * t44324;
    let t44328 = F::new(0.28455006635676149599e-1) * t2268 * t2440 * t3518;
    let t44329 = t44268 * t447;
    let t44334 = F::new(0.37940008847568199465e-1) * t6313 * t13319;
    (t44319, t44322, t44325, t44328, t44329, t44334)
}
