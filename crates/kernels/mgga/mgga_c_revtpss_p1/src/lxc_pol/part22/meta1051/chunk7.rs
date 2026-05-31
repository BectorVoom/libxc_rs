//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3712/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3712<F: Float>(t482: F, t69623: F, t1042: F, t1261: F, t17202: F, t17448: F, t17558: F, t17569: F, t17669: F, t17796: F, t3610: F, t3611: F, t44170: F, t44343: F, t44698: F, t5381: F, t5407: F, t56254: F, t57098: F, t57100: F, t57114: F, t58983: F, t65829: F, t65947: F, t6631: F, t6635: F) -> (F, F) {
    let t70343 = t482 * t69623;
    let t70361 = -F::cast_from(0.85748036236139473944e-3_f64) * t1261 * t1042 * t17202 * t65829 - F::cast_from(0.34299214494455789578e-2_f64) * t1261 * t1042 * t56254 * t65947 + F::cast_from(0.47637797908966374413e-3_f64) * t5381 * t17558 + F::cast_from(0.42874018118069736972e-3_f64) * t44170 * t6631 - F::cast_from(0.21437009059034868486e-3_f64) * t44698 * t6635 - F::cast_from(0.42874018118069736972e-3_f64) * t3610 * t1042 * t70343 * t3611 - F::cast_from(0.47637797908966374413e-3_f64) * t17569 * t17796 + F::cast_from(0.23289590088828005269e-2_f64) * t1261 * t1042 * t58983 * t65947 + F::cast_from(0.3811023832717309953e-3_f64) * t57098 - F::cast_from(0.57165357490759649296e-3_f64) * t57100 * t5407 - F::cast_from(0.57165357490759649296e-3_f64) * t17448 * t17669 - F::cast_from(0.3811023832717309953e-3_f64) * t57114 + t44343 / F::cast_from(648.0_f64);
    (t70343, t70361)
}
