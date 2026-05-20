//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2972/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2972<F: Float>(t11875: F, t11922: F, t15898: F, t1011: F, t16003: F, t16006: F, t3241: F, t42712: F, t42716: F, t42719: F, t42724: F, t42727: F, t42740: F, t42745: F, t4919: F, t51873: F) -> F {
    let t54187 = t11875 * t11922 * t15898;
    let t54195 = F::new(2.0) / F::new(9.0) * t3241 * t16003 + t1011 * t4919 * t51873 / F::new(6.0) - F::new(2.0) / F::new(27.0) * t3241 * t16006 + F::cast_from(0.42874018118069736972e-3_f64) * t54187 + t42712 / F::new(81.0) + F::new(5.0) / F::new(1296.0) * t42716 + t42719 / F::new(216.0) + F::new(11.0) / F::new(324.0) * t42724 + t42727 / F::new(144.0) - F::new(5.0) / F::new(162.0) * t42740 - t42745;
    t54195
}
