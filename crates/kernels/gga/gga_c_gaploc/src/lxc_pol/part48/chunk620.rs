//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 620/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk620<F: Float>(t12804: F, t2321: F, t3556: F, t882: F, t3565: F, t888: F, t2268: F, t3560: F, t11271: F, t3340: F, t999: F, t3518: F, t894: F, t13296: F, t493: F, t492: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13303 = 0.94850022118920498664e-2 * t12804;
    let t13304 = t3556 * t2321;
    let t13305 = t882 * t13304;
    let t13306 = 0.11856252764865062333e-2 * t13305;
    let t13307 = t3565 * t888;
    let t13309 = 0.19918504644973304719e0 * t2268 * t13307;
    let t13310 = t3560 * t2321;
    let t13311 = t882 * t13310;
    let t13312 = 0.11856252764865062333e-2 * t13311;
    let t13313 = t11271 * t888;
    let t13315 = 0.85365019907028448797e-1 * t2268 * t13313;
    let t13316 = t999 * t3340;
    let t13319 = t894 * t3518;
    let t13321 = 0.28455006635676149599e-1 * t2268 * t13319;
    let t13322 = t493 * t13296;
    let t13323 = t492 * t13322;
    (t13303, t13304, t13306, t13307, t13309, t13310, t13312, t13313, t13315, t13316, t13319, t13321, t13322, t13323)
}
