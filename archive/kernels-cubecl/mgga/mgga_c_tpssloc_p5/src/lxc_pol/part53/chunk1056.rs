//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1056/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1056<F: Float>(t100993: F, t117390: F, t120818: F, t124587: F, t12524: F, t1458: F, t20173: F, t2039: F, t24462: F, t24465: F, t27170: F, t27273: F, t27276: F, t27281: F, t31287: F, t32295: F, t33192: F, t34099: F, t3941: F, t4072: F, t55353: F, t577: F, t7056: F, t7235: F, t7801: F, t7956: F, t84033: F, t8717: F, t94170: F) -> F {
    let t124668 = F::cast_from(54.0_f64) * t24465 * t27281 + F::cast_from(54.0_f64) * t94170 * t7235 + F::cast_from(0.135e2_f64) * t32295 * t4072 + F::cast_from(54.0_f64) * t20173 * t34099 + F::cast_from(54.0_f64) * t3941 * t7056 * t7801 + F::cast_from(54.0_f64) * t3941 * t2039 * t27170 + F::cast_from(0.135e2_f64) * t117390 * t1458 + t120818 + F::cast_from(54.0_f64) * t100993 * t7956 + F::cast_from(54.0_f64) * t84033 * t7956 + F::cast_from(27.0_f64) * t55353 * t8717 + F::cast_from(0.45e1_f64) * t124587 * t577 + F::cast_from(54.0_f64) * t24465 * t27273 + F::cast_from(54.0_f64) * t24465 * t27276 + t31287 + t33192 + F::cast_from(54.0_f64) * t12524 * t34099 + F::cast_from(27.0_f64) * t24462 * t7801;
    t124668
}
