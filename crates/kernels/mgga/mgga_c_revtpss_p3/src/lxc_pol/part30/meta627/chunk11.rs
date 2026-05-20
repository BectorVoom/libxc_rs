//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2185/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2185<F: Float>(t15071: F, t1544: F, t1583: F, t18875: F, t1940: F, t2403: F, t2430: F, t25436: F, t25440: F, t27158: F, t27364: F, t27368: F, t27375: F, t2832: F, t4343: F, t4537: F, t51780: F, t61102: F, t61203: F, t63186: F, t7087: F, t7091: F, t775: F, t7783: F, t7847: F, t890: F, t92775: F, t98651: F, t99555: F) -> F {
    let t100926 = -t15071 * t1940 * t7091 + F::new(3.0) * t1544 * t2403 * t25436 - t1583 * t1940 * t92775 - F::new(6.0) * t18875 * t2403 * t25440 - F::new(2.0) * t1940 * t25440 * t4537 - t1940 * t27368 * t2832 - F::new(2.0) * t1940 * t890 * t99555 + F::new(3.0) * t2403 * t2430 * t7783 - F::new(6.0) * t2403 * t25440 * t27375 + F::new(6.0) * t2403 * t27364 * t775 + F::new(6.0) * t2403 * t4343 * t7087 - F::new(6.0) * t2403 * t61102 * t7091 - F::new(3.0) * t2403 * t61203 * t7091 - F::new(3.0) * t2403 * t7091 * t98651 - F::new(12.0) * t27158 * t63186 + F::new(6.0) * t51780 * t7847;
    t100926
}
