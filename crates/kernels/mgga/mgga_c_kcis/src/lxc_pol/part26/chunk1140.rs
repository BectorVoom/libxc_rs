//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1140/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1140<F: Float>(t28744: F, t28797: F, t28840: F, t28867: F, t12933: F, t12940: F, t1629: F, t2128: F, t27710: F, t28557: F, t28559: F, t28560: F, t28579: F, t28645: F, t28655: F, t28658: F, t28663: F, t28666: F, t28698: F, t4475: F, t4480: F, t6222: F, t633: F, t8010: F, t8240: F, t8251: F) -> (F, F) {
    let t28869 = t28744 + t28797 + t28840 + t28867;
    let t28873 = F::cast_from(2.0_f64) * t12933 * t8240 - F::cast_from(6.0_f64) * t12940 * t28655 - t1629 * t28698 - t2128 * t27710 + F::cast_from(2.0_f64) * t28658 * t4480 + F::cast_from(2.0_f64) * t28663 * t4480 + F::cast_from(2.0_f64) * t28666 * t4480 + t28869 * t633 - t4475 * t8251 - t6222 * t8010 - t28557 + t28559 + t28560 + t28579 + t28645;
    (t28869, t28873)
}
