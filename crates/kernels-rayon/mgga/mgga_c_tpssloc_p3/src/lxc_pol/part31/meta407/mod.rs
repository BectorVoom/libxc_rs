//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1498;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1499;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta407(t562: f64, t6414: f64, t5250: f64, t12171: f64, t6388: f64, t3901: f64, t6415: f64, t11984: f64, t15880: f64, t15889: f64, t15894: f64, t19543: f64, t19574: f64, t19576: f64, t19581: f64, t19588: f64, t19589: f64, t19590: f64, t19592: f64, t19594: f64, t9457: f64, t9476: f64, t9484: f64, t12050: f64, t12091: f64, t12044: f64, t12048: f64, t12057: f64, t12059: f64, t12087: f64, t12094: f64, t15898: f64, t15911: f64, t15916: f64, t15917: f64, t15923: f64, t19599: f64, t9780: f64, t9789: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19660, t19661, t19668, t19674, t19676) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1498(t562, t6414, t5250, t12171, t6388, t3901, t6415, t11984, t15880, t15889, t15894, t19543, t19574, t19576, t19581, t19588, t19589, t19590, t19592, t19594, t9457, t9476, t9484);
        let (t19677, t19678, t19679) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1499(t12050, t12091, t12044, t12048, t12057, t12059, t12087, t12094, t15898, t15911, t15916, t15917, t15923, t19599, t9780, t9789);
    (t19660, t19661, t19668, t19674, t19676, t19677, t19678, t19679)
}
