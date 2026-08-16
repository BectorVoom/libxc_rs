//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3849/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3849<F: Float>(t22263: F, t9775: F, t1412: F, t6861: F, t2661: F, t3938: F, t3992: F, t5608: F, t5659: F, t1399: F, t22025: F, t1353: F, t13902: F, t13907: F, t1392: F, t1394: F, t21969: F, t22229: F, t22237: F, t22240: F, t22245: F, t22246: F, t22249: F, t3829: F, t3889: F, t4045: F, t539: F, t5644: F, t5650: F, t5652: F, t6837: F, t6840: F, t73: F, t73578: F, t73991: F, t74012: F) -> (F, F, F, F, F, F) {
    let t74024 = t9775 * t22263;
    let t74026 = t1412 * t6861;
    let t74029 = t2661 * t3992 * t74026 * t3938;
    let t74033 = t2661 * t3992 * t5608 * t5659;
    let t74037 = t2661 * t3992 * t22025 * t1399;
    let t74077 = -F::cast_from(24.0_f64) * t1353 * t1412 * t21969 * t5650 + F::cast_from(3.0_f64) * t1394 * t539 * t73578 - F::cast_from(12.0_f64) * t22245 * t3889 * t5650 - F::cast_from(360.0_f64) * t3829 * t5650 * t73991 + F::cast_from(60.0_f64) * t3829 * t5650 * t74012 - F::cast_from(48.0_f64) * t5644 * t5652 * t73 + F::cast_from(120.0_f64) * t13902 * t22237 - F::cast_from(48.0_f64) * t13902 * t22240 - F::cast_from(24.0_f64) * t13902 * t22246 + F::cast_from(120.0_f64) * t13907 * t22229 + F::cast_from(6.0_f64) * t1392 * t22249 - F::cast_from(12.0_f64) * t4045 * t6837 + F::cast_from(3.0_f64) * t4045 * t6840;
    (t74024, t74026, t74029, t74033, t74037, t74077)
}
