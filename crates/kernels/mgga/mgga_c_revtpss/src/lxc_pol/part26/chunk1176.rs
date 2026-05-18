//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1176/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1176<F: Float>(t93058: F, t93063: F, t93067: F, t93069: F, t93073: F, t93075: F, t93077: F, t93080: F, t93084: F, t93086: F, t93088: F, t93091: F, t93093: F, t93095: F) -> F {
    let t95713 = -F::new(0.15246000842785598468e-3) * t93058 - F::new(0.51448821741683684367e-2) * t93063 - F::new(0.27210710165601593065e0) * t93067 + F::new(0.48018900292238105409e-1) * t93069 + F::new(0.65049603595885220128e-2) * t93073 - F::new(0.34299214494455789578e-2) * t93075 - F::new(0.6098400337114239387e-3) * t93077 + F::new(0.85748036236139473944e-4) * t93080 - F::new(0.17149607247227894789e-3) * t93084 - F::new(0.24009450146119052704e0) * t93086 - F::new(0.91464571985215438874e-3) * t93088 + F::new(0.85748036236139473944e-4) * t93091 - F::new(0.10289764348336736873e0) * t93093 + F::new(0.30492001685571196935e-2) * t93095;
    t95713
}
