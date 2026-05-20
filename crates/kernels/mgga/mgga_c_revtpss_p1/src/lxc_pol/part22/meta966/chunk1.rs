//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3230/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3230<F: Float>(t11075: F, t14468: F, t1544: F, t18268: F, t18850: F, t198: F, t2393: F, t2394: F, t2403: F, t2430: F, t4541: F, t4542: F, t49950: F, t5966: F, t61234: F, t61240: F, t61244: F, t61245: F, t61248: F, t61249: F, t61250: F, t61261: F) -> F {
    let t61262 = F::new(6.0) * t11075 * t4541 * t5966 + F::new(12.0) * t14468 * t4541 * t4542 + F::new(6.0) * t1544 * t2403 * t49950 - F::new(3.0) * t18268 * t2403 * t2430 + F::new(6.0) * t18850 * t2394 * t4541 + F::new(12.0) * t198 * t2393 * t61234 - t61240 + t61244 + t61245 + t61248 + t61249 + t61250 + t61261;
    t61262
}
