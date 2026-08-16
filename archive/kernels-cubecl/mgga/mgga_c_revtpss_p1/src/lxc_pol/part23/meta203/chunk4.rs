//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1211/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1211<F: Float>(t1399: F, t1437: F, t1883: F, t213: F, t4082: F, t4085: F, t4090: F, t4094: F, t4099: F, t4105: F, t4109: F, t4113: F, t4118: F, t546: F, t5659: F, t5675: F, t5710: F, t5735: F, t5738: F, t5742: F, t5745: F, t5755: F, t5761: F, t5765: F, t5767: F, t820: F) -> F {
    let t5774 = t4082 - t4085 + F::cast_from(0.54878743191129263322e-2_f64) * t4090 - F::cast_from(0.54878743191129263322e-2_f64) * t4094 + t4099 - F::cast_from(0.9757440539382783019e-2_f64) * t4105 + F::cast_from(0.9757440539382783019e-2_f64) * t4109 - t4113 + F::cast_from(0.54878743191129263322e-2_f64) * t5738 - F::cast_from(0.9757440539382783019e-2_f64) * t5742 + F::cast_from(0.13170898365871023197e1_f64) * t5745 * t5735 * t5675 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t4118 * t1883 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t1437 * t5659 - F::cast_from(0.65854491829355115987e0_f64) * t5755 * t5735 * t1399 - F::cast_from(0.54878743191129263322e-2_f64) * t5761 + F::cast_from(0.9757440539382783019e-2_f64) * t5765 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t5767 * t1399 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t546 * t5710;
    t5774
}
