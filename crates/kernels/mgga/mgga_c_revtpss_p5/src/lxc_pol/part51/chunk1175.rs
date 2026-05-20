//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1175/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1175<F: Float>(t119675: F, t119737: F, t125997: F, t126411: F, t1544: F, t1583: F, t18875: F, t1940: F, t198: F, t207: F, t2403: F, t27375: F, t27384: F, t31859: F, t31863: F, t33727: F, t4343: F, t4433: F, t4537: F, t4541: F, t775: F, t8490: F, t890: F, t892: F) -> F {
    let t127143 = t126411 * t198 * t207 * t892 + F::new(2.0) * t119675 * t1940 * t27384 - t119737 * t1583 * t1940 - t125997 * t1940 * t890 + F::new(3.0) * t1544 * t2403 * t31859 - F::new(3.0) * t18875 * t2403 * t31863 - t1940 * t31863 * t4537 - F::new(3.0) * t2403 * t27375 * t31863 + F::new(3.0) * t2403 * t33727 * t775 + F::new(3.0) * t2403 * t4343 * t8490 + F::new(6.0) * t4433 * t4541 * t8490;
    t127143
}
