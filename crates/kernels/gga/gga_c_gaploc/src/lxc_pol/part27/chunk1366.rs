//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1366/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1366<F: Float>(t12002: F, t12012: F, t12019: F, t1212: F, t1349: F, t1353: F, t1358: F, t1359: F, t13777: F, t161: F, t31646: F, t31652: F, t31660: F, t31662: F, t31672: F, t31674: F, t31679: F, t31681: F, t31685: F, t3692: F, t380: F, t419: F, t488: F) -> F {
    let t38354 = F::cast_from(0.56910013271352299198e-1_f64) * t419 * t12019 - F::cast_from(0.7588001769513639893e-1_f64) * t380 * t12002 + F::cast_from(0.28455006635676149599e-1_f64) * t1212 * t3692 - F::cast_from(0.63233348079280332442e-2_f64) * t1358 * t1359 * t13777 * t488 + F::cast_from(0.63233348079280332442e-2_f64) * t1349 * t12012 * t161 * t1353 - t31646 - t31652 - t31660 + t31662 - t31672 - t31674 + t31679 + t31681 + t31685;
    t38354
}
