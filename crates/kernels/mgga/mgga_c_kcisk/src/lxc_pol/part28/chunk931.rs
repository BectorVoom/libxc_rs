//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 931/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk931<F: Float>(t18442: F, t2013: F, t2020: F, t7233: F, t4998: F, t7610: F, t7614: F, t10886: F, t7605: F, t240: F, t6847: F, t15991: F, t16398: F, t1965: F, t7464: F, t2597: F, t5397: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18443 = t2013 * t18442;
    let t18445 = t7233 * t2020;
    let t18451 = t4998 * t7610;
    let t18453 = 0.59969295720591057378e-2 * t2013 * t18451;
    let t18454 = t4998 * t7614;
    let t18456 = 0.11993859144118211476e-1 * t2013 * t18454;
    let t18457 = t10886 * t7605;
    let t18458 = t2013 * t18457;
    let t18472 = t240 * t6847;
    let t18499 = 0.22954444444444444444e0 * t15991;
    let t18514 = 0.27785333333333333334e0 * t16398;
    let t18541 = t7464 * t1965;
    let t18546 = t2597 * t5397;
    (t18443, t18445, t18453, t18456, t18458, t18472, t18499, t18514, t18541, t18546)
}
