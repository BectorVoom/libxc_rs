//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1555;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1556;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta435(t19680: f64, t4806: f64, t1042: f64, t5819: f64, t999: f64, t1032: f64, t6235: f64, t1040: f64, t5825: f64, t4872: f64, t1651: f64, t905: f64, t4873: f64, t3092: f64, t357: f64, t4866: f64, t4893: f64, t3117: f64, t19450: f64, t4900: f64, t11661: f64, t19501: f64, t1047: f64, t1063: f64, t12013: f64, t16067: f64, t16089: f64, t3127: f64, t4803: f64, t4808: f64, t4834: f64, t4892: f64, t4899: f64, t6308: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19688, t19691, t19693, t19696, t19697, t19702, t19705) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1555(t19680, t4806, t1042, t5819, t999, t1032, t6235, t1040, t5825, t4872, t1651, t905);
        let (t19707, t19718, t19722, t19726, t19729) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1556(t19705, t4873, t3092, t357, t4866, t4893, t3117, t19450, t4900, t11661, t19501, t1047, t1063, t12013, t16067, t16089, t19688, t19693, t19697, t19702, t3127, t4803, t4808, t4834, t4892, t4899, t6308);
    (t19688, t19691, t19693, t19696, t19702, t19707, t19718, t19722, t19726, t19729)
}
