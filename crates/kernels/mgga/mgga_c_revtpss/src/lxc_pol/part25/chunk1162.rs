//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1162/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1162<F: Float>(t14365: F, t94245: F, t11054: F, t33: F, t25759: F, t41161: F, t1113: F, t2394: F, t1940: F, t1963: F, t2403: F, t25206: F, t25436: F, t25440: F, t25760: F, t25763: F, t25784: F, t27158: F, t27382: F, t3351: F, t4541: F, t7087: F, t7091: F, t92819: F, t93397: F, t9357: F, t94228: F, t94231: F, t94234: F, t94240: F) -> (F,) {
    let t94246 = t94245 * t14365;
    let t94255 = t33 * t11054;
    let t94259 = t25759 * t41161;
    let t94262 = t1113 * t2394;
    let t94272 = 3.0 / 2.0 * t1940 * t25436 * t1113 - 9.0 / 2.0 * t25206 * t94228 + 9.0 * t25206 * t94231 + 3.0 * t27382 * t94234 + t1940 * t1963 * t9357 / 2.0 - 9.0 * t27158 * t94240 - 9.0 * t92819 * t25760 - 9.0 * t25206 * t94246 + 3.0 / 2.0 * t1940 * t7087 * t3351 + t1940 * t93397 * t33 / 2.0 - t1940 * t7091 * t94255 / 2.0 - 9.0 / 2.0 * t25206 * t94259 + 9.0 * t4541 * t1963 * t94262 + 9.0 * t2403 * t7087 * t25763 - 3.0 / 2.0 * t1940 * t25440 * t25784;
    (t94272,)
}
