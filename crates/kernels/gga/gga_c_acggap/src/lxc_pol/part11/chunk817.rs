//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 817/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk817<F: Float>(t7674: F, t8835: F, t8839: F, t8841: F, t8843: F, t8845: F, t8847: F, t8849: F, t8851: F, t8856: F, t8860: F, t8862: F, t8864: F, t8866: F, t8870: F) -> F {
    let t8872 = -t7674 + F::cast_from(0.20007875121765877254e-2_f64) * t8835 - F::cast_from(0.53592522647587171215e-3_f64) * t8839 + F::cast_from(0.85748036236139473944e-3_f64) * t8841 + F::cast_from(0.85748036236139473944e-3_f64) * t8843 + F::cast_from(0.85748036236139473944e-3_f64) * t8845 - F::cast_from(0.85748036236139473944e-3_f64) * t8847 - F::cast_from(0.85748036236139473944e-3_f64) * t8849 + F::cast_from(0.10718504529517434243e-3_f64) * t8851 + F::cast_from(0.10718504529517434243e-3_f64) * t8856 + F::cast_from(0.7145669686344956162e-4_f64) * t8860 + F::cast_from(0.18868855373762491241e-2_f64) * t8862 - t8864 / F::cast_from(96.0_f64) - t8866 / F::cast_from(48.0_f64) + F::cast_from(0.15724046144802076034e-3_f64) * t8870;
    t8872
}
