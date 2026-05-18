//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1013/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1013<F: Float>(t29343: F, t29425: F, t29451: F, t29466: F, t3: F, t1461: F, t1918: F, t2170: F, t28257: F, t28259: F, t28261: F, t28263: F, t28267: F, t28270: F, t28273: F, t28275: F, t28279: F, t28282: F, t573: F, t5802: F, t5805: F, t7696: F, t8245: F) -> (F, F, F) {
    let t29468 = t29343 + t29425 + t29451 + t29466;
    let t29469 = t3 * t29468;
    let t29480 = param_d * t29468;
    let t29490 = F::new(3.0) * t1461 * t8245 + F::new(3.0) * t1918 * t7696 + F::new(6.0) * t2170 * t5802 + F::new(3.0) * t2170 * t5805 + t29480 * t573 + t28257 + t28259 + t28261 + t28263 + t28267 + t28270 + t28273 + t28275 + t28279 + t28282;
    (t29469, t29480, t29490)
}
