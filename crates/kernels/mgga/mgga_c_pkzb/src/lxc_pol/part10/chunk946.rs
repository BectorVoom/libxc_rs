//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 946/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk946<F: Float>(t7335: F, t5522: F, t7332: F, t7352: F, t7361: F, t7363: F, t7366: F, t7368: F, t7371: F, t7373: F, t7376: F, t7379: F, t7386: F, t7389: F, t5525: F, t5560: F, t5563: F, t5566: F, t5783: F, t5790: F, t7357: F, t7393: F, t7397: F, t7401: F) -> (F, F, F, F, F) {
    let t7420 = 0.59793333333333333334e0 * t7335;
    let t7431 = 0.27385555555555555555e0 * t7332 - t7420 + 0.8969e0 * t7352 + 0.3071625e0 * t7361 + 0.1898925e1 * t7363 - 0.1898925e1 * t7366 - 0.9494625e0 * t7368 + 0.3071625e0 * t7371 + 0.15358125e0 * t7373 + 0.142419375e1 * t7376 - 0.76790625e-1 * t7379 + 0.79724444444444444446e0 * t5522;
    let t7434 = 0.32862666666666666666e0 * t7386;
    let t7435 = 0.32862666666666666666e0 * t7389;
    let t7442 = -0.29896666666666666667e0 * t5525 + 0.39862222222222222223e0 * t7357 - t7434 - t7435 + 0.24647e0 * t7393 + 0.49294e0 * t7397 + 0.24647e0 * t7401 - t5783 - t5790 + 0.54771111111111111111e0 * t5560 - 0.16431333333333333333e0 * t5563 - 0.16431333333333333333e0 * t5566;
    (t7420, t7431, t7434, t7435, t7442)
}
