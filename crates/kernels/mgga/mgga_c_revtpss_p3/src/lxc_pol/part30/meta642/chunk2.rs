//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2239/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2239<F: Float>(t3666: F, t8184: F, t17307: F, t2138: F, t17451: F, t26867: F, t1285: F, t97173: F, t104646: F, t17735: F, t1238: F, t16715: F, t17502: F, t17541: F, t17584: F, t17635: F, t17696: F, t17739: F, t26880: F, t29047: F, t3674: F, t5279: F, t57549: F, t97250: F) -> (F, F) {
    let t104924 = t3666 * t8184;
    let t104927 = t17307 * t2138;
    let t104933 = t26867 * t17451;
    let t104943 = t1285 * t97173;
    let t104946 = t17735 * t104646;
    let t104951 = F::cast_from(0.45732285992607719436e-2_f64) * t104924 * t1238 + F::cast_from(0.85748036236139473944e-3_f64) * t104927 * t3674 - F::new(7.0) / F::new(648.0) * t29047 * t57549 * t16715 - F::cast_from(0.3811023832717309953e-3_f64) * t104933 + F::cast_from(0.28582678745379824648e-3_f64) * t26880 * t17541 + F::cast_from(0.57165357490759649296e-3_f64) * t97250 * t5279 + F::cast_from(0.57165357490759649296e-3_f64) * t26880 * t17502 + F::cast_from(0.28582678745379824648e-3_f64) * t26880 * t17584 + F::cast_from(0.95275595817932748826e-3_f64) * t104943 * t17696 - F::cast_from(0.11433071498151929859e-2_f64) * t104946 * t17739 - F::cast_from(0.57165357490759649296e-3_f64) * t26867 * t17635;
    (t104943, t104951)
}
