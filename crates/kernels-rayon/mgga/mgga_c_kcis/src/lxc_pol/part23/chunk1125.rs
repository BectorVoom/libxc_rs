//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1125/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1125(t28744: f64, t28797: f64, t28840: f64, t28867: f64, t12933: f64, t12940: f64, t1629: f64, t2128: f64, t27710: f64, t28557: f64, t28559: f64, t28560: f64, t28579: f64, t28645: f64, t28655: f64, t28658: f64, t28663: f64, t28666: f64, t28698: f64, t4475: f64, t4480: f64, t6222: f64, t633: f64, t8010: f64, t8240: f64, t8251: f64) -> (f64, f64) {
    let t28869 = t28744 + t28797 + t28840 + t28867;
    let t28873 = 2.0_f64 * t12933 * t8240 - 6.0_f64 * t12940 * t28655 - t1629 * t28698 - t2128 * t27710 + 2.0_f64 * t28658 * t4480 + 2.0_f64 * t28663 * t4480 + 2.0_f64 * t28666 * t4480 + t28869 * t633 - t4475 * t8251 - t6222 * t8010 - t28557 + t28559 + t28560 + t28579 + t28645;
    (t28869, t28873)
}
