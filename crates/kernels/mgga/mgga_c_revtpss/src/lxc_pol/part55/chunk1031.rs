//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1031/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1031<F: Float>(t122463: F, t32710: F, t32705: F, t121211: F, t32685: F, t689: F, t121131: F, t121365: F, t121227: F, t121272: F, t121275: F, t121099: F, t32275: F, t32707: F, t121307: F, t121342: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t122464 = t32710 * t122463;
    let t122466 = t32705 * t122463;
    let t122468 = 0.47023883532522246276e-4 * t121211;
    let t122474 = t32685 * t689;
    let t122475 = t121131 * t122474;
    let t122477 = t121365 * t122474;
    let t122480 = 0.39666484489654411541e-3 * t121227;
    let t122493 = 0.7437465841810202164e-5 * t121272;
    let t122494 = 0.39671442800215618342e-4 * t121275;
    let t122496 = t121099 * t32275 * t32707;
    let t122498 = 0.40155686056505553065e-3 * t121307;
    let t122503 = 0.71396809808466873356e-3 * t121342;
    (t122464, t122466, t122468, t122475, t122477, t122480, t122493, t122494, t122496, t122498, t122503)
}
