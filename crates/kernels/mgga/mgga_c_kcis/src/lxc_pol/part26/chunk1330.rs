//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1330/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1330<F: Float>(t101826: F, t101828: F, t101830: F, t101832: F, t101833: F, t101835: F, t101837: F, t101862: F, t101898: F, t101936: F, t101971: F, t101991: F, t102025: F, t102045: F, t102074: F, t102098: F, t102131: F, t102164: F, t102200: F, t102239: F, t102269: F, t102310: F, t102342: F, t102374: F, t102400: F, t102427: F, t102459: F, t102488: F, t102509: F, t102535: F, t102558: F, t102594: F, t102623: F, t102646: F, t102671: F, t102696: F, t102727: F, t102751: F, t102775: F, t12940: F, t1636: F, t2128: F, t23272: F, t27702: F, t29499: F, t29502: F, t29652: F, t40653: F, t4480: F, t52930: F, t6225: F, t6256: F, t63256: F, t633: F, t7537: F, t8001: F, t8010: F, t8240: F, t99718: F, t99730: F) -> F {
    let t102804 = -F::new(12.0) * t12940 * t8240 * t6256 - t101826 + t101828 + (t101991 + t102074 + t102045 + t102727 + t102775 + t102558 + t102131 + t102535 + t102488 + t102310 + t102098 + t102342 + t101936 + t102400 + t102269 + t102025 + t102623 + t102239 + t102671 + t102200 + t102374 + t102459 + t102509 + t101862 + t102751 + t101898 + t102646 + t102164 + t102696 + t102427 + t101971 + t102594) * t633 + F::new(4.0) * t52930 * t8240 + t101830 - F::new(12.0) * t12940 * t29499 * t1636 + F::new(2.0) * t63256 * t8001 + t101832 + t101833 - t101835 + F::new(2.0) * t4480 * t29652 * t1636 + F::new(4.0) * t99718 * t6225 - F::new(6.0) * t12940 * t8010 * t7537 + F::new(24.0) * t40653 * t29502 * t1636 - F::new(2.0) * t99730 * t2128 + t101837 + F::new(2.0) * t27702 * t23272;
    t102804
}
