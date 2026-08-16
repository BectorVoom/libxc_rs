//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3674/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3674(t1180: f64, t1188: f64, t12423: f64, t12470: f64, t16948: f64, t16951: f64, t16955: f64, t16959: f64, t16962: f64, t17023: f64, t17032: f64, t17085: f64, t20537: f64, t20622: f64, t20625: f64, t20626: f64, t3453: f64, t3471: f64, t3477: f64, t3491: f64, t3497: f64, t3521: f64, t3523: f64, t45080: f64, t45157: f64, t45159: f64, t45168: f64, t45188: f64, t45190: f64, t5125: f64, t5146: f64, t5147: f64, t58304: f64, t58317: f64, t58336: f64, t6486: f64, t6518: f64, t6538: f64, t68598: f64, t68795: f64, t69090: f64) -> f64 {
    let t69467 = 0.12865583598954028054e3_f64 * t12423 * t20622 + 0.64327917994770140268e2_f64 * t3477 * t5146 * t17085 + 0.4138081033541872024e4_f64 * t45080 * t20626 + 0.2069040516770936012e4_f64 * t12470 * t20625 * t3471 + 0.19964560303604640732e6_f64 * t45157 * t6486 * t45159 * t3453 + t69090 + 0.91082604192152556044e5_f64 * t45188 * t6518 * t45190 * t3497 + 0.11696447245269292414e1_f64 * t3491 * t20537 + 0.5848223622634646207e0_f64 * t1180 * t68598 * t1188 + 0.17315859105681463759e2_f64 * t45168 * t6538 + 0.34631718211362927518e2_f64 * t3521 * t68795 * t3523 - 8.0_f64 * t58336 * t5125 - 8.0_f64 * t17023 * t16948 - 4.0_f64 * t17023 * t16951 - 0.38596750796862084161e3_f64 * t58304 * t16955 + 0.12865583598954028054e3_f64 * t58317 * t5147 + 0.12865583598954028054e3_f64 * t17032 * t16959 + 0.64327917994770140268e2_f64 * t17032 * t16962;
    t69467
}
