//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1069/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1069<F: Float>(t2113: F, t7334: F, t1459: F, t8731: F, t1936: F, t28974: F, t572: F, t26733: F, t7002: F, t7553: F, t10301: F, t8736: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32764 = F::cast_from(3.0_f64) * t2113 * t7334;
    let t32772 = F::cast_from(6.0_f64) * t1459 * t8731;
    let t32773 = t28974 * t1936;
    let t32775 = F::cast_from(6.0_f64) * t572 * t32773;
    let t32776 = t26733 * t1936;
    let t32778 = F::cast_from(6.0_f64) * t572 * t32776;
    let t32779 = t7553 * t7002;
    let t32781 = F::cast_from(6.0_f64) * t572 * t32779;
    let t32795 = t10301 * t8736;
    (t32764, t32772, t32773, t32775, t32776, t32778, t32779, t32781, t32795)
}
