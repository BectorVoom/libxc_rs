//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 927/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk927<F: Float>(t10023: F, t10024: F, t3295: F, t7354: F, t2684: F, t2365: F, t7069: F, t7390: F, t2440: F, t988: F, t2268: F, t2756: F, t894: F) -> (F, F, F, F, F, F, F, F) {
    let t10026 = F::new(0.89376224879626066674e-1) * t10023 * t10024;
    let t10029 = t7354 * t3295;
    let t10030 = t2684 * t10029;
    let t10031 = F::new(0.51123901271894332901e0) * t10030;
    let t10040 = t2365 * t7069;
    let t10042 = F::new(0.29792074959875355558e-1) * t7390 * t10040;
    let t10113 = t2440 * t988;
    let t10115 = F::new(0.28455006635676149599e-1) * t2268 * t10113;
    let t10116 = t894 * t2756;
    (t10026, t10029, t10031, t10040, t10042, t10113, t10115, t10116)
}
