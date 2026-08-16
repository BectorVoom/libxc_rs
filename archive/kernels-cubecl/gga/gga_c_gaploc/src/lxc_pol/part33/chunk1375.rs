//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1375/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1375<F: Float>(t12148: F, t1377: F, t13777: F, t493: F, t1339: F, t1436: F, t1441: F, t1537: F, t1540: F, t18383: F, t34025: F, t34032: F, t34036: F, t34038: F, t34052: F, t34054: F, t34056: F, t34058: F, t34061: F, t34065: F, t34067: F, t4130: F, t4781: F) -> (F, F) {
    let t38458 = F::cast_from(2.0_f64) * t1377 * t12148;
    let t38463 = t493 * t13777;
    let t38474 = -t34025 + F::cast_from(0.30674340763136599742e1_f64) * t4781 * t4130 * t13777 * t1540 - F::cast_from(0.1022478025437886658e1_f64) * t1436 * t38463 * t18383 + F::cast_from(0.2044956050875773316e1_f64) * t1441 * t38463 * t1540 - F::cast_from(0.51123901271894332902e1_f64) * t1537 * t1339 * t13777 * t1540 - t34032 - t34036 - t34038 - t34052 - t34054 - t34056 + t34058 + t34061 - t34065 + t34067;
    (t38458, t38474)
}
