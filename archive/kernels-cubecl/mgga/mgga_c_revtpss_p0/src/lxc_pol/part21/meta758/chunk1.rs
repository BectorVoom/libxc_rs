//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2667/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2667<F: Float>(t1398: F, t5591: F, t124: F, t1370: F, t13783: F, t13804: F, t221: F, t3889: F, t3934: F, t3936: F, t3938: F, t47296: F, t47298: F, t47302: F, t47304: F, t47306: F, t48421: F, t49062: F, t49066: F, t49071: F, t49085: F, t49087: F, t49090: F, t49093: F, t49103: F, t49105: F, t5627: F, t5671: F, t5704: F, t800: F, t9912: F, t9995: F) -> F {
    let t49107 = t5591 * t1398;
    let t49112 = -F::cast_from(0.60023625365297631762e-2_f64) * t49062 + F::cast_from(0.85748036236139473944e-4_f64) * t49066 - t49071 + F::cast_from(0.51448821741683684367e-2_f64) * t13804 * t3936 * t5704 * t9995 - F::cast_from(0.51448821741683684367e-2_f64) * t5671 * t3936 * t5704 * t9912 - F::cast_from(0.76230004213927992336e-4_f64) * t47296 + F::cast_from(0.40656002247428262581e-3_f64) * t47298 + F::cast_from(0.76230004213927992336e-3_f64) * t47302 - F::cast_from(0.17006693853500995666e-1_f64) * t47304 + F::cast_from(0.10003937560882938627e-2_f64) * t47306 - F::cast_from(0.24009450146119052704e-1_f64) * t49085 - F::cast_from(0.18295201011342718161e-3_f64) * t49087 + F::cast_from(0.3252886739816735289e-3_f64) * t49090 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t49093 * t221 * t5627 * t3889 - t1370 * t800 * t124 * t48421 / F::cast_from(48.0_f64) + F::cast_from(0.27107389498472794075e-4_f64) * t49103 + F::cast_from(0.45178982497454656792e-6_f64) * t49105 - F::cast_from(0.25724410870841842183e-1_f64) * t3934 * t13783 * t49107 * t3938;
    t49112
}
