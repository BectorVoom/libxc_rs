//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 838/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk838<F: Float>(t2316: F, t2636: F, t3378: F, t1081: F, t2804: F, t3375: F, t9673: F, t320: F, t8700: F, t3379: F, t3402: F, t8838: F) -> (F, F, F, F, F) {
    let t10018 = t2636 * t2316;
    let t10019 = t3378 * t10018;
    let t10021 = t1081 * t2804;
    let t10024 = t9673 * t3375;
    let t10026 = t320 * t8700;
    let t10027 = t10026 * t3379;
    let t10029 = t3402 * t8838;
    (t10019, t10021, t10024, t10027, t10029)
}
