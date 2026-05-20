//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 918/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk918<F: Float>(t27553: F, t27592: F, t27650: F, t27706: F, t3336: F, t7840: F, t1100: F, t1699: F, t1544: F, t1583: F, t18875: F, t1940: F, t1963: F, t198: F, t207: F, t2403: F, t25440: F, t25445: F, t27363: F, t27368: F, t27375: F, t27384: F, t4343: F, t4433: F, t4537: F, t4541: F, t7087: F, t7091: F, t775: F, t7783: F, t890: F, t892: F) -> (F, F, F, F) {
    let t27708 = t27553 + t27592 + t27650 + t27706;
    let t27712 = t7840 * t3336;
    let t27717 = t1699 * t1100;
    let t27754 = t198 * t207 * t27363 * t892 + F::new(3.0) * t1544 * t2403 * t7087 - t1583 * t1940 * t25440 - F::new(3.0) * t18875 * t2403 * t7091 + F::new(2.0) * t1940 * t25445 * t27384 - t1940 * t27368 * t890 - t1940 * t4537 * t7091 + F::new(3.0) * t1963 * t2403 * t4343 + F::new(6.0) * t1963 * t4433 * t4541 - F::new(3.0) * t2403 * t27375 * t7091 + F::new(3.0) * t2403 * t775 * t7783;
    (t27708, t27712, t27717, t27754)
}
