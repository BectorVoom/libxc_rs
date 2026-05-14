//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 697/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk697<F: Float>(t13728: F, t2343: F, t2268: F, t11977: F, t888: F, t3691: F, t894: F, t11986: F, t2325: F, t883: F, t882: F, t12764: F, t12769: F, t12774: F, t12802: F, t12809: F, t12836: F, t12838: F, t12842: F, t13726: F) -> (F, F, F, F, F) {
    let t13729 = t2343 * t13728;
    let t13730 = t2268 * t13729;
    let t13732 = t11977 * t888;
    let t13733 = t2268 * t13732;
    let t13735 = t894 * t3691;
    let t13736 = t2268 * t13735;
    let t13740 = t2325 * t883 * t11986;
    let t13741 = t882 * t13740;
    let t13745 = 0.11856252764865062333e-2 * t13726 + 0.56910013271352299198e-1 * t13730 - 0.85365019907028448797e-1 * t13733 + 0.28455006635676149599e-1 * t13736 + t12836 + 0.28455006635676149599e-1 * t12838 - t12842 - 0.11856252764865062333e-2 * t13741 + 0.56910013271352299198e-1 * t12764 + t12769 - 0.85365019907028448797e-1 * t12774 - t12802 - t12809;
    (t13729, t13732, t13735, t13740, t13745)
}
