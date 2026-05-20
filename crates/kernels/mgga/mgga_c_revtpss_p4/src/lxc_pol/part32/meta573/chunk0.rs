//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1897/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1897<F: Float>(t102100: F, t25944: F, t25950: F, t28845: F, t28780: F, t94886: F, t28889: F, t686: F, t72: F, t7284: F, t10073: F, t1903: F, t2102: F, t25929: F) -> (F, F, F, F, F, F) {
    let t102101 = t25944 * t102100;
    let t102104 = F::cast_from(0.25702851531048074406e-1_f64) * t25950 * t28845;
    let t102113 = F::cast_from(0.51405703062096148812e-1_f64) * t94886 * t28780;
    let t102115 = t28889 * t72 * t686;
    let t102117 = F::cast_from(0.14456046980341999104e-1_f64) * t7284 * t102115;
    let t102120 = t10073 * t25929 * t2102 * t1903;
    (t102101, t102104, t102113, t102115, t102117, t102120)
}
