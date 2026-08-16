//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2667/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2667(t1398: f64, t5591: f64, t124: f64, t1370: f64, t13783: f64, t13804: f64, t221: f64, t3889: f64, t3934: f64, t3936: f64, t3938: f64, t47296: f64, t47298: f64, t47302: f64, t47304: f64, t47306: f64, t48421: f64, t49062: f64, t49066: f64, t49071: f64, t49085: f64, t49087: f64, t49090: f64, t49093: f64, t49103: f64, t49105: f64, t5627: f64, t5671: f64, t5704: f64, t800: f64, t9912: f64, t9995: f64) -> f64 {
    let t49107 = t5591 * t1398;
    let t49112 = -0.60023625365297631762e-2_f64 * t49062 + 0.85748036236139473944e-4_f64 * t49066 - t49071 + 0.51448821741683684367e-2_f64 * t13804 * t3936 * t5704 * t9995 - 0.51448821741683684367e-2_f64 * t5671 * t3936 * t5704 * t9912 - 0.76230004213927992336e-4_f64 * t47296 + 0.40656002247428262581e-3_f64 * t47298 + 0.76230004213927992336e-3_f64 * t47302 - 0.17006693853500995666e-1_f64 * t47304 + 0.10003937560882938627e-2_f64 * t47306 - 0.24009450146119052704e-1_f64 * t49085 - 0.18295201011342718161e-3_f64 * t49087 + 0.3252886739816735289e-3_f64 * t49090 - 3.0_f64 / 4.0_f64 * t49093 * t221 * t5627 * t3889 - t1370 * t800 * t124 * t48421 / 48.0_f64 + 0.27107389498472794075e-4_f64 * t49103 + 0.45178982497454656792e-6_f64 * t49105 - 0.25724410870841842183e-1_f64 * t3934 * t13783 * t49107 * t3938;
    t49112
}
