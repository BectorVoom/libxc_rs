//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3511/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3511<F: Float>(t3151: F, t6258: F, t11922: F, t19744: F, t3115: F, t20104: F, t11703: F, t11875: F, t13396: F, t15139: F, t15153: F, t15950: F, t16095: F, t19750: F, t19754: F, t2852: F, t3117: F, t3162: F, t4181: F, t42410: F, t42656: F, t4573: F, t4772: F, t53654: F, t53657: F, t54099: F, t54118: F, t54122: F, t55011: F) -> (F, F) {
    let t66341 = t6258 * t3151;
    let t66355 = t3115 * t11922 * t19744;
    let t66362 = t3115 * t11922 * t20104;
    let t66373 = F::cast_from(0.25724410870841842184e-2_f64) * t53654 * t19750 - F::cast_from(0.25724410870841842184e-2_f64) * t53657 * t19754 + F::cast_from(0.21437009059034868486e-3_f64) * t11875 * t3117 * t66341 * t3162 - F::cast_from(0.57165357490759649297e-2_f64) * t55011 * t11703 * t15153 * t13396 - F::cast_from(0.95275595817932748828e-3_f64) * t16095 * t11703 * t4573 * t15950 - F::cast_from(0.28582678745379824648e-3_f64) * t66355 + F::cast_from(0.2540682555144873302e-2_f64) * t55011 * t42410 * t15139 * t13396 - F::cast_from(0.28582678745379824648e-3_f64) * t66362 - F::cast_from(0.95275595817932748826e-3_f64) * t16095 * t11703 * t4772 * t2852 * t4181 - F::cast_from(0.3811023832717309953e-3_f64) * t54099 + F::cast_from(0.5081365110289746604e-3_f64) * t42656 + F::cast_from(5.0_f64) / F::cast_from(1944.0_f64) * t54118 + t54122 / F::cast_from(162.0_f64);
    (t66341, t66373)
}
