//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1815/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1815<F: Float>(t1312: F, t1518: F, t18245: F, t2055: F, t28653: F, t30138: F, t30143: F, t30553: F, t30570: F, t30589: F, t4248: F, t5920: F, t7359: F, t7889: F, t7983: F) -> F {
    let t30612 = F::cast_from(2.0_f64) * t1312 * t30570 + F::cast_from(4.0_f64) * t1518 * t28653 + F::cast_from(2.0_f64) * t18245 * t2055 + F::cast_from(4.0_f64) * t2055 * t30138 + F::cast_from(2.0_f64) * t2055 * t30143 + F::cast_from(4.0_f64) * t4248 * t7983 + F::cast_from(2.0_f64) * t5920 * t7359 + F::cast_from(4.0_f64) * t7889 * t7983 + t30553 + F::cast_from(2.0_f64) * t30589;
    t30612
}
