//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 845/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk845<F: Float>(t10178: F, t9689: F, t3889: F, t566: F, t1343: F, t1353: F, t1450: F, t198: F, t4139: F, t4140: F, t532: F, t5536: F, t9524: F, t9542: F, t9590: F, t9593: F, t9598: F, t9599: F, t9628: F, t9854: F, t9857: F, t9859: F, t9862: F, t9865: F, t9868: F) -> (F, F) {
    let t10179 = t9689 + t10178;
    let t10186 = t566 * t3889;
    let t10190 = t10179 * t1450 * t198 * t532 + F::cast_from(2.0_f64) * t198 * t532 * t9590 * t9593 + F::cast_from(18.0_f64) * t10186 * t1353 * t5536 + F::cast_from(3.0_f64) * t1343 * t198 * t9628 - F::cast_from(9.0_f64) * t1353 * t4139 * t9599 + F::cast_from(9.0_f64) * t3889 * t4139 * t4140 - t9524 + t9542 + t9598 + t9854 - t9857 - t9859 + t9862 + t9865 + t9868;
    (t10179, t10190)
}
