//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1368/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1368<F: Float>(t114216: F, t114221: F, t114230: F, t114238: F, t114407: F, t114410: F, t114415: F, t114417: F, t114419: F, t114421: F, t114427: F, t1502: F, t1518: F, t1843: F, t2127: F, t25043: F, t25045: F, t30724: F, t30944: F, t30951: F, t34446: F, t4248: F, t5920: F, t5921: F, t651: F, t6765: F, t7586: F, t8152: F, t8233: F) -> F {
    let t116722 = -F::cast_from(6.0_f64) * t1518 * t30944 * t651 - F::cast_from(6.0_f64) * t5920 * t651 * t8233 - F::cast_from(3.0_f64) * t1502 * t30944 - F::cast_from(6.0_f64) * t1843 * t30724 - t2127 * t25043 - F::cast_from(6.0_f64) * t25045 * t7586 - F::cast_from(6.0_f64) * t30951 * t4248 - F::cast_from(6.0_f64) * t34446 * t5921 - F::cast_from(3.0_f64) * t6765 * t8152 - t114216 + t114221 - t114230 - t114238 - t114407 - t114410 - t114415 - t114417 - t114419 - t114421 + t114427;
    t116722
}
