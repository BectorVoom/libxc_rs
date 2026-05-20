//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3515/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3515<F: Float>(t13392: F, t15787: F, t15936: F, t16020: F, t16048: F, t16052: F, t16095: F, t16096: F, t16584: F, t18941: F, t19572: F, t19738: F, t19754: F, t20066: F, t20094: F, t20099: F, t2857: F, t3092: F, t3117: F, t4181: F, t42712: F, t42716: F, t42719: F, t4772: F, t4899: F, t4902: F, t54023: F, t54187: F) -> F {
    let t66535 = F::cast_from(0.45732285992607719436e-2_f64) * t16584 * t16048 * t4902 + F::cast_from(0.85748036236139473944e-3_f64) * t19738 * t15787 - F::cast_from(0.45732285992607719436e-2_f64) * t16052 * t20066 + F::cast_from(0.57165357490759649296e-3_f64) * t16095 * t3092 * t18941 * t16096 + F::cast_from(0.11433071498151929859e-2_f64) * t16095 * t3092 * t4772 * t2857 * t4181 - F::cast_from(0.17149607247227894789e-2_f64) * t16095 * t3092 * t20099 * t15936 + F::cast_from(0.57165357490759649296e-3_f64) * t16095 * t3092 * t20094 * t13392 + F::cast_from(0.13719685797782315831e-1_f64) * t54023 * t19754 - F::cast_from(0.21437009059034868486e-3_f64) * t4899 * t3117 * t19572 * t16020 + F::cast_from(0.28582678745379824648e-3_f64) * t54187 + t42712 / F::new(243.0) + F::new(5.0) / F::new(1944.0) * t42716 + t42719 / F::new(648.0);
    t66535
}
