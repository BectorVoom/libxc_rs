//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 688/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk688<F: Float>(t3599: F, t921: F, t1382: F, t11402: F, t895: F, t11386: F, t1645: F, t2492: F, t11359: F, t13276: F, t1445: F, t1562: F) -> (F, F, F, F, F, F, F, F) {
    let t13350 = t3599 * t921;
    let t13352 = F::cast_from(2.0_f64) * t1382 * t13350;
    let t13354 = F::cast_from(0.35750489951850426669e0_f64) * t895 * t11402;
    let t13356 = F::cast_from(0.35750489951850426669e0_f64) * t895 * t11386;
    let t13363 = t1645 * t2492;
    let t13365 = F::cast_from(0.42900587942220512003e1_f64) * t11359 * t13363;
    let t13368 = t1445 * t13276;
    let t13370 = F::cast_from(0.62115540045351614476e2_f64) * t1562 * t13368;
    (t13350, t13352, t13354, t13356, t13363, t13365, t13368, t13370)
}
