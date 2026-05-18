//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1151/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1151<F: Float>(t2407: F, t469: F, t301: F, t694: F, t2248: F, t3952: F, t11179: F, t1679: F, t467: F, t103: F, t10761: F, t2254: F, t2408: F, t32276: F, t33358: F, t33383: F, t33397: F, t36647: F, t5439: F, t567: F, t642: F, t7288: F, t7297: F, t7301: F, t8027: F, t8040: F, t8372: F, t9096: F, t9098: F, t9121: F, t922: F, t9460: F) -> F {
    let t36686 = t2407 * t469;
    let t36689 = F::new(6.0) * t694 * t36686 * t301;
    let t36706 = t2248 * t3952;
    let t36715 = F::new(2.0) * t1679 * t11179 * t467;
    let t36716 = t103 * t2407;
    let t36726 = F::new(6.0) * t2254 * t567 * t642 * t922 - F::new(6.0) * t10761 * t5439 * t7297 + F::new(2.0) * t2408 * t567 * t8027 + F::new(12.0) * t33358 * t7297 * t9460 + F::new(2.0) * t33383 * t9096 * t9460 - F::new(12.0) * t33397 * t8040 * t8372 - F::new(3.0) * t36647 * t7297 * t8040 + F::new(4.0) * t36706 * t9096 * t9098 + F::new(6.0) * t36716 * t567 * t7288 + F::new(3.0) * t567 * t7301 * t9121 - t32276 + t36689 - t36715;
    t36726
}
