//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1130/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1130<F: Float>(t104115: F, t111734: F, t124169: F, t128240: F, t128242: F, t128244: F, t128245: F, t128251: F, t128254: F, t128256: F, t128260: F, t128261: F, t128266: F, t128270: F, t128273: F, t130928: F, t130929: F, t130932: F, t130946: F, t1518: F, t2055: F, t29427: F, t32175: F, t32177: F, t33287: F, t33645: F, t4292: F, t569: F, t670: F, t7367: F, t8563: F) -> (F,) {
    let t130951 = -2.0 * t29427 * t7367 + t128240 + t128242 - t128244 - t128245 - t128251 - t128254 - t128256 + t128260 - t128261 - t128266 + t128270 - t128273 + (2.0 * t104115 * t2055 + 2.0 * t111734 * t2055 + 2.0 * t124169 * t1518 + 2.0 * t130929 * t670 + 2.0 * t130932 * t1518 + 2.0 * t33287 * t4292 + t130928 + 2.0 * t130946 + 2.0 * t32175 + 2.0 * t32177 + 2.0 * t33645 + 2.0 * t8563) * t569;
    (t130951,)
}
