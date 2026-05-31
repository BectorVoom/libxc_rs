//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2150/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2150<F: Float>(t106554: F, t106561: F, t106565: F, t106625: F, t1544: F, t18435: F, t18838: F, t18875: F, t1940: F, t1963: F, t2403: F, t25445: F, t27364: F, t27368: F, t27375: F, t29705: F, t29907: F, t4537: F, t4541: F, t50080: F, t5966: F, t6079: F, t7087: F, t7091: F, t77425: F, t77441: F, t775: F, t92742: F, t93404: F) -> F {
    let t107867 = F::cast_from(4.0_f64) * t106554 * t1940 * t25445 + F::cast_from(6.0_f64) * t106561 * t2403 * t25445 - F::cast_from(6.0_f64) * t106565 * t1940 * t92742 - F::cast_from(6.0_f64) * t106625 * t2403 * t7091 + F::cast_from(6.0_f64) * t1544 * t2403 * t27364 + F::cast_from(6.0_f64) * t18435 * t1963 * t4541 - t18838 * t1940 * t7091 - F::cast_from(6.0_f64) * t18875 * t2403 * t27368 - F::cast_from(2.0_f64) * t1940 * t27368 * t4537 + F::cast_from(2.0_f64) * t1940 * t6079 * t93404 - F::cast_from(6.0_f64) * t2403 * t27368 * t27375 + F::cast_from(3.0_f64) * t2403 * t29705 * t775 - F::cast_from(3.0_f64) * t2403 * t7091 * t77425 - F::cast_from(6.0_f64) * t2403 * t7091 * t77441 + F::cast_from(6.0_f64) * t4541 * t5966 * t7087 + F::cast_from(6.0_f64) * t29907 * t50080;
    t107867
}
