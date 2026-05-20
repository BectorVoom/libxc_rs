//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2040/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2040<F: Float>(t102888: F, t107901: F, t107919: F, t107924: F, t107930: F, t107988: F, t108009: F, t108030: F, t110177: F, t110717: F, t1113: F, t1940: F, t2071: F, t2403: F, t26425: F, t27793: F, t28291: F, t28472: F, t29953: F, t29964: F, t30420: F, t4541: F, t6416: F, t7207: F, t7428: F, t7432: F, t95976: F) -> F {
    let t110954 = t1940 * t95976 * t29964 + F::new(2.0) * t28472 * t107924 - F::new(3.0) / F::new(2.0) * t26425 * t107919 - F::new(3.0) * t102888 * t27793 - F::new(3.0) * t26425 * t107930 + F::new(3.0) * t28291 * t108030 + t1940 * t7428 * t6416 / F::new(2.0) + t1940 * t30420 * t1113 / F::new(2.0) + F::new(3.0) * t2403 * t2071 * t107901 + F::new(3.0) / F::new(2.0) * t2403 * t7428 * t29953 - t1940 * t7432 * t107988 + F::new(3.0) * t4541 * t2071 * t108009 - t1940 * t110177 * t7207 / F::new(2.0) - t110717;
    t110954
}
