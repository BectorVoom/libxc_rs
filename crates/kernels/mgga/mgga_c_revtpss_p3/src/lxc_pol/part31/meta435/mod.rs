//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1555;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1556;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta435<F: Float>(t19680: F, t4806: F, t1042: F, t5819: F, t999: F, t1032: F, t6235: F, t1040: F, t5825: F, t4872: F, t1651: F, t905: F, t4873: F, t3092: F, t357: F, t4866: F, t4893: F, t3117: F, t19450: F, t4900: F, t11661: F, t19501: F, t1047: F, t1063: F, t12013: F, t16067: F, t16089: F, t3127: F, t4803: F, t4808: F, t4834: F, t4892: F, t4899: F, t6308: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19688, t19691, t19693, t19696, t19697, t19702, t19705) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1555::<F>(t19680, t4806, t1042, t5819, t999, t1032, t6235, t1040, t5825, t4872, t1651, t905);
        let (t19707, t19718, t19722, t19726, t19729) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1556::<F>(t19705, t4873, t3092, t357, t4866, t4893, t3117, t19450, t4900, t11661, t19501, t1047, t1063, t12013, t16067, t16089, t19688, t19693, t19697, t19702, t3127, t4803, t4808, t4834, t4892, t4899, t6308);
    (t19688, t19691, t19693, t19696, t19702, t19707, t19718, t19722, t19726, t19729)
}
