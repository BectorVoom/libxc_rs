//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3518/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3518<F: Float>(t15905: F, t56017: F, t55899: F, t11703: F, t11859: F, t15606: F, t15609: F, t15908: F, t15910: F, t16020: F, t16025: F, t16067: F, t16084: F, t16095: F, t16096: F, t18936: F, t19450: F, t19501: F, t19572: F, t19758: F, t19954: F, t3117: F, t3241: F, t42675: F, t43044: F, t4891: F, t4902: F, t53669: F, t54314: F, t54324: F, t54570: F, t55985: F, t64891: F) -> F {
    let t66621 = t56017 * t15905;
    let t66624 = t55899 * t15905;
    let t66631 = -F::new(4.0) / F::new(243.0) * t54314 - F::new(4.0) / F::new(81.0) * t3241 * t19954 + F::cast_from(0.85748036236139473944e-3_f64) * t54570 * t15606 - F::cast_from(0.85748036236139473944e-3_f64) * t11859 * t3117 * t19572 * t15609 + F::cast_from(0.21437009059034868486e-3_f64) * t16067 * t3117 * t19450 * t16020 - F::cast_from(0.47637797908966374414e-3_f64) * t16095 * t11703 * t18936 * t16096 + F::cast_from(0.30011812682648815881e-2_f64) * t53669 * t3117 * t64891 * t15908 - F::cast_from(0.42874018118069736972e-3_f64) * t43044 * t3117 * t19501 * t16025 - F::cast_from(0.22866142996303859718e-2_f64) * t42675 * t19758 + F::cast_from(0.25724410870841842183e-2_f64) * t66621 * t16084 - F::cast_from(0.25724410870841842183e-2_f64) * t66624 * t15910 + F::cast_from(0.30488190661738479624e-2_f64) * t54324 - F::cast_from(0.85748036236139473944e-3_f64) * t55985 * t4891 * t4902;
    t66631
}
