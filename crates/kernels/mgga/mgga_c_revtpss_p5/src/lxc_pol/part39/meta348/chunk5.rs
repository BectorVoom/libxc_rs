//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1179/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1179<F: Float>(t14019: F, t800: F, t5686: F, t9744: F, t1353: F, t5689: F, t1872: F, t3889: F, t1370: F, t14007: F, t14013: F, t14016: F, t3944: F, t9748: F, t9924: F, t9926: F, t9932: F, t9937: F, t9953: F) -> F {
    let t14020 = t800 * t14019;
    let t14024 = F::new(7.0) / F::new(24.0) * t9744 * t5686;
    let t14026 = t800 * t5689 * t1353;
    let t14030 = t800 * t1872 * t3889;
    let t14033 = -t14007 + F::cast_from(0.25410001404642664112e-3_f64) * t9924 + F::cast_from(0.40015750243531754508e-2_f64) * t9926 + F::cast_from(0.71456696863449561619e-5_f64) * t9932 - F::cast_from(0.14291339372689912324e-4_f64) * t9937 - F::cast_from(0.18071592998981862717e-4_f64) * t14013 - t9748 * t14016 / F::new(4.0) - t1370 * t14020 / F::new(48.0) - t14024 + t3944 * t14026 / F::new(8.0) + t3944 * t14030 / F::new(16.0) - t9953;
    t14033
}
