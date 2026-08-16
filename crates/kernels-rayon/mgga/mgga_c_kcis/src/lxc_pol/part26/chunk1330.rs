//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1330/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1330(t101826: f64, t101828: f64, t101830: f64, t101832: f64, t101833: f64, t101835: f64, t101837: f64, t101862: f64, t101898: f64, t101936: f64, t101971: f64, t101991: f64, t102025: f64, t102045: f64, t102074: f64, t102098: f64, t102131: f64, t102164: f64, t102200: f64, t102239: f64, t102269: f64, t102310: f64, t102342: f64, t102374: f64, t102400: f64, t102427: f64, t102459: f64, t102488: f64, t102509: f64, t102535: f64, t102558: f64, t102594: f64, t102623: f64, t102646: f64, t102671: f64, t102696: f64, t102727: f64, t102751: f64, t102775: f64, t12940: f64, t1636: f64, t2128: f64, t23272: f64, t27702: f64, t29499: f64, t29502: f64, t29652: f64, t40653: f64, t4480: f64, t52930: f64, t6225: f64, t6256: f64, t63256: f64, t633: f64, t7537: f64, t8001: f64, t8010: f64, t8240: f64, t99718: f64, t99730: f64) -> f64 {
    let t102804 = -12.0_f64 * t12940 * t8240 * t6256 - t101826 + t101828 + (t101991 + t102074 + t102045 + t102727 + t102775 + t102558 + t102131 + t102535 + t102488 + t102310 + t102098 + t102342 + t101936 + t102400 + t102269 + t102025 + t102623 + t102239 + t102671 + t102200 + t102374 + t102459 + t102509 + t101862 + t102751 + t101898 + t102646 + t102164 + t102696 + t102427 + t101971 + t102594) * t633 + 4.0_f64 * t52930 * t8240 + t101830 - 12.0_f64 * t12940 * t29499 * t1636 + 2.0_f64 * t63256 * t8001 + t101832 + t101833 - t101835 + 2.0_f64 * t4480 * t29652 * t1636 + 4.0_f64 * t99718 * t6225 - 6.0_f64 * t12940 * t8010 * t7537 + 24.0_f64 * t40653 * t29502 * t1636 - 2.0_f64 * t99730 * t2128 + t101837 + 2.0_f64 * t27702 * t23272;
    t102804
}
