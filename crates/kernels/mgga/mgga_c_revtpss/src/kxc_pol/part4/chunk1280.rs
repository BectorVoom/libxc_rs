//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1280/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1280<F: Float>(t1011: F, t15988: F, t11672: F, t11675: F, t11881: F, t11886: F, t12004: F, t15952: F, t15959: F, t15965: F, t15970: F, t15975: F, t15986: F, t1675: F, t3091: F, t3127: F, t4783: F, t4892: F, t4899: F) -> F {
    let t15990 = t1011 * t15988 / F::cast_from(216.0_f64);
    let t15991 = -F::cast_from(0.28582678745379824648e-3_f64) * t3127 * t15952 + F::cast_from(0.28582678745379824648e-3_f64) * t11675 * t4783 + F::cast_from(0.28582678745379824648e-3_f64) * t3091 * t15959 - F::cast_from(0.28582678745379824648e-3_f64) * t3091 * t15965 + F::cast_from(0.28582678745379824648e-3_f64) * t4892 * t15970 - F::cast_from(0.14291339372689912324e-3_f64) * t4899 * t15975 + F::cast_from(0.48272968547752592739e-2_f64) * t12004 * t1675 - t11881 / F::cast_from(648.0_f64) - t11886 / F::cast_from(162.0_f64) - F::cast_from(0.15244095330869239812e-2_f64) * t11672 * t4783 + t15986 - t15990;
    t15991
}
