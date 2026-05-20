//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2035/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2035<F: Float>(t106554: F, t106565: F, t106610: F, t107793: F, t107805: F, t110698: F, t18435: F, t18498: F, t18838: F, t1940: F, t198: F, t207: F, t2071: F, t2403: F, t26425: F, t26585: F, t26590: F, t27375: F, t28291: F, t28460: F, t30420: F, t4541: F, t5962: F, t6075: F, t7428: F, t7432: F, t77408: F, t77425: F, t77441: F, t775: F, t892: F, t95964: F) -> F {
    let t110792 = -t1940 * t26585 * t6075 + F::new(3.0) * t2403 * t30420 * t775 + F::new(3.0) * t2403 * t7428 * t5962 + F::new(12.0) * t4541 * t2071 * t18498 - t1940 * t7432 * t18838 - F::new(3.0) * t2403 * t7432 * t77425 + F::new(2.0) * t1940 * t26590 * t106610 + F::new(6.0) * t4541 * t2071 * t18435 - F::new(6.0) * t2403 * t7432 * t77441 + F::new(4.0) * t1940 * t26590 * t106554 - F::new(6.0) * t4541 * t7432 * t77408 - F::new(6.0) * t2403 * t28460 * t27375 + t198 * t207 * t110698 * t892 - F::new(12.0) * t28291 * t107793 + F::new(12.0) * t26425 * t107805 - F::new(6.0) * t1940 * t95964 * t106565;
    t110792
}
