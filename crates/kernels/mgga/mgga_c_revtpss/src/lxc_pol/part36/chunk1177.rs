//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1177/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1177<F: Float>(t1963: F, t5966: F, t1544: F, t1583: F, t1940: F, t198: F, t207: F, t2403: F, t25445: F, t27368: F, t29598: F, t29704: F, t4541: F, t5962: F, t6075: F, t6079: F, t7091: F, t7783: F, t892: F) -> F {
    let t29907 = t1963 * t5966;
    let t29930 = t198 * t207 * t29704 * t892 + F::new(6.0) * t1544 * t2403 * t7783 - F::new(2.0) * t1583 * t1940 * t27368 + F::new(2.0) * t1940 * t25445 * t6079 - t1940 * t6075 * t7091 + F::new(3.0) * t1963 * t2403 * t5962 - F::new(6.0) * t2403 * t29598 * t7091 + F::new(6.0) * t29907 * t4541;
    t29930
}
