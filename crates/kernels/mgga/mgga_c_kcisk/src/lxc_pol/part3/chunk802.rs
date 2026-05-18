//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 802/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk802<F: Float>(t12377: F, t12396: F, t11690: F, t11693: F, t11696: F, t11698: F, t11704: F, t11707: F, t11936: F, t12340: F, t12342: F, t12345: F, t12352: F, t12353: F, t12356: F, t2042: F, t2049: F, t5527: F, t5532: F, t5533: F, t5552: F, t802: F) -> F {
    let t12397 = t12377 + t12396;
    let t12399 = t12340 * t802 - F::new(3.0) * t12342 * t2049 + F::new(6.0) * t12345 * t5533 - F::new(6.0) * t12352 * t12353 + F::new(6.0) * t12356 * t5532 - t12397 * t2042 - F::new(3.0) * t5527 * t5552 - t11690 + t11693 - t11696 + t11698 + t11704 - t11707 + t11936;
    t12399
}
