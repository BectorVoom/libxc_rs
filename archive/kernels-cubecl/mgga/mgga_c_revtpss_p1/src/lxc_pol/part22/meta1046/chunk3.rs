//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3674/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3674<F: Float>(t1180: F, t1188: F, t12423: F, t12470: F, t16948: F, t16951: F, t16955: F, t16959: F, t16962: F, t17023: F, t17032: F, t17085: F, t20537: F, t20622: F, t20625: F, t20626: F, t3453: F, t3471: F, t3477: F, t3491: F, t3497: F, t3521: F, t3523: F, t45080: F, t45157: F, t45159: F, t45168: F, t45188: F, t45190: F, t5125: F, t5146: F, t5147: F, t58304: F, t58317: F, t58336: F, t6486: F, t6518: F, t6538: F, t68598: F, t68795: F, t69090: F) -> F {
    let t69467 = F::cast_from(0.12865583598954028054e3_f64) * t12423 * t20622 + F::cast_from(0.64327917994770140268e2_f64) * t3477 * t5146 * t17085 + F::cast_from(0.4138081033541872024e4_f64) * t45080 * t20626 + F::cast_from(0.2069040516770936012e4_f64) * t12470 * t20625 * t3471 + F::cast_from(0.19964560303604640732e6_f64) * t45157 * t6486 * t45159 * t3453 + t69090 + F::cast_from(0.91082604192152556044e5_f64) * t45188 * t6518 * t45190 * t3497 + F::cast_from(0.11696447245269292414e1_f64) * t3491 * t20537 + F::cast_from(0.5848223622634646207e0_f64) * t1180 * t68598 * t1188 + F::cast_from(0.17315859105681463759e2_f64) * t45168 * t6538 + F::cast_from(0.34631718211362927518e2_f64) * t3521 * t68795 * t3523 - F::cast_from(8.0_f64) * t58336 * t5125 - F::cast_from(8.0_f64) * t17023 * t16948 - F::cast_from(4.0_f64) * t17023 * t16951 - F::cast_from(0.38596750796862084161e3_f64) * t58304 * t16955 + F::cast_from(0.12865583598954028054e3_f64) * t58317 * t5147 + F::cast_from(0.12865583598954028054e3_f64) * t17032 * t16959 + F::cast_from(0.64327917994770140268e2_f64) * t17032 * t16962;
    t69467
}
