//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1250/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1250<F: Float>(t1181: F, t3361: F, t5087: F, t530: F, t1861: F, t3670: F, t13788: F, t13791: F, t17619: F, t17621: F, t17623: F, t17627: F, t17631: F, t17635: F, t17650: F, t17661: F) -> F {
    let t22993 = t3361 * t1181 * t530 * t5087;
    let t22995 = t3670 * t1861;
    let t23003 = F::cast_from(0.85748036236139473944e-3_f64) * t17619 + F::cast_from(0.17149607247227894789e-2_f64) * t17621 + F::cast_from(0.85748036236139473944e-3_f64) * t17623 - F::cast_from(0.51448821741683684367e-2_f64) * t17627 + F::cast_from(0.34299214494455789578e-2_f64) * t22993 - F::cast_from(0.90702367218671976884e-1_f64) * t22995 + F::cast_from(0.40015750243531754508e-2_f64) * t17631 + F::cast_from(0.13719685797782315831e-1_f64) * t17635 + F::new(35.0) / F::new(72.0) * t13788 + F::new(35.0) / F::new(216.0) * t13791 - F::cast_from(0.68598428988911579156e-2_f64) * t17650 - F::cast_from(0.40015750243531754508e-1_f64) * t17661;
    t23003
}
