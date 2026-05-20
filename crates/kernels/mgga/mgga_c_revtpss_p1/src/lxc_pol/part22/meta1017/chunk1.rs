//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3517/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3517<F: Float>(t19477: F, t3153: F, t1011: F, t15926: F, t15950: F, t16012: F, t16045: F, t16089: F, t19705: F, t19809: F, t3092: F, t3117: F, t3241: F, t42781: F, t42785: F, t4772: F, t4873: F, t4899: F, t4900: F, t4919: F, t54261: F, t54303: F, t54306: F, t63258: F, t63283: F, t63288: F, t905: F) -> (F, F) {
    let t66565 = t19477 * t3153;
    let t66591 = F::cast_from(0.30488190661738479624e-2_f64) * t54261 + F::cast_from(0.11433071498151929859e-2_f64) * t16089 * t3092 * t4772 * t905 * t4873 - F::cast_from(0.42874018118069736972e-3_f64) * t4899 * t3117 * t66565 * t4900 + F::cast_from(0.84689418504829110067e-4_f64) * t42781 + F::cast_from(0.6351706387862183255e-4_f64) * t42785 + F::cast_from(0.11433071498151929859e-2_f64) * t16089 * t3092 * t19705 * t15950 - F::cast_from(0.42874018118069736972e-3_f64) * t15926 * t16045 + t1011 * t4919 * t63258 / F::new(108.0) + t1011 * t4919 * t63283 / F::new(216.0) + F::new(7.0) / F::new(648.0) * t1011 * t16012 * t63288 + F::new(4.0) / F::new(27.0) * t3241 * t19809 + F::new(2.0) / F::new(81.0) * t54303 + t54306 / F::new(72.0);
    (t66565, t66591)
}
