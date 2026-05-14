//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1024/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1024<F: Float>(t4962: F, t9938: F, t991: F, t1071: F, t1704: F, t2630: F, t2894: F, t1000: F, t4951: F, t1003: F, t4621: F, t13475: F, t4947: F, t291: F, t13511: F, t2872: F, t4963: F, t9883: F, t9906: F, t9910: F, t9918: F, t9940: F, t9957: F, t9961: F, t9970: F) -> (F,) {
    let t14536 = t9938 * t4962;
    let t14538 = t991 * t14536 / 432.0;
    let t14542 = t1704 * t1071 * t2630;
    let t14543 = t2894 * t14542;
    let t14546 = t4951 * t1000;
    let t14547 = t4621 * t1003;
    let t14548 = t14546 * t14547;
    let t14551 = t4947 * t13475;
    let t14554 = t4951 * t291;
    let t14555 = t14554 * t13511;
    let t14561 = t9883 - t9906 / 162.0 - t9910 / 432.0 - t9918 / 648.0 - t9940 / 432.0 - t14538 + t2872 * t4963 / 54.0 + t991 * t14543 / 144.0 - t991 * t14548 / 72.0 - t991 * t14551 / 144.0 - t991 * t14555 / 36.0 + t9957 / 864.0 + t9961 / 648.0 + t9970 / 81.0;
    (t14561,)
}
