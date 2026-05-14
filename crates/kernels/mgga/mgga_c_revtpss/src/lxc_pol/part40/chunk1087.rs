//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1087/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1087<F: Float>(t1872: F, t3889: F, t800: F, t1370: F, t14007: F, t14013: F, t14016: F, t14020: F, t14024: F, t14026: F, t3944: F, t9748: F, t9924: F, t9926: F, t9932: F, t9937: F, t9953: F) -> (F,) {
    let t14030 = t800 * t1872 * t3889;
    let t14033 = -t14007 + 0.25410001404642664112e-3 * t9924 + 0.40015750243531754508e-2 * t9926 + 0.71456696863449561619e-5 * t9932 - 0.14291339372689912324e-4 * t9937 - 0.18071592998981862717e-4 * t14013 - t9748 * t14016 / 4.0 - t1370 * t14020 / 48.0 - t14024 + t3944 * t14026 / 8.0 + t3944 * t14030 / 16.0 - t9953;
    (t14033,)
}
