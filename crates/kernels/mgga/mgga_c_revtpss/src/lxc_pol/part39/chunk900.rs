//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 900/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk900<F: Float>(t1427: F, t5774: F, t1424: F, t1445: F, t1904: F, t213: F, t3894: F, t3898: F, t3901: F, t3904: F, t3910: F, t3912: F, t3918: F, t3922: F, t4071: F, t5601: F, t5604: F, t561: F, t5711: F, t5715: F, t5719: F, t5723: F, t5728: F) -> (F, F) {
    let t5775 = t1427 * t5774;
    let t5778 = t3894 - t3898 - 0.54878743191129263322e-2 * t3901 + 0.54878743191129263322e-2 * t3904 + t3910 + 0.9757440539382783019e-2 * t3912 - 0.9757440539382783019e-2 * t3918 - t3922 - 0.54878743191129263322e-2 * t5601 + 0.9757440539382783019e-2 * t5604 + 0.65854491829355115987e0 * t213 * t5711 * t561 - 0.65854491829355115987e0 * t5715 * t1445 + 0.54878743191129263322e-2 * t5719 - 0.9757440539382783019e-2 * t5723 - 0.65854491829355115987e0 * t4071 * t1904 + 0.13170898365871023197e1 * t1424 * t5728 - 0.65854491829355115987e0 * t1424 * t5775;
    (t5775, t5778)
}
