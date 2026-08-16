//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta427 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1651;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1652;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta427<F: Float>(t562: F, t6414: F, t5250: F, t12171: F, t6388: F, t3901: F, t6415: F, t11984: F, t15880: F, t15889: F, t15894: F, t19543: F, t19574: F, t19576: F, t19581: F, t19588: F, t19589: F, t19590: F, t19592: F, t19594: F, t9457: F, t9476: F, t9484: F, t12050: F, t12091: F, t12044: F, t12048: F, t12057: F, t12059: F, t12087: F, t12094: F, t15898: F, t15911: F, t15916: F, t15917: F, t15923: F, t19599: F, t9780: F, t9789: F) -> (F, F, F, F, F, F, F, F) {
        let (t19660, t19661, t19668, t19674, t19676) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1651::<F>(t562, t6414, t5250, t12171, t6388, t3901, t6415, t11984, t15880, t15889, t15894, t19543, t19574, t19576, t19581, t19588, t19589, t19590, t19592, t19594, t9457, t9476, t9484);
        let (t19677, t19678, t19679) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1652::<F>(t12050, t12091, t12044, t12048, t12057, t12059, t12087, t12094, t15898, t15911, t15916, t15917, t15923, t19599, t9780, t9789);
    (t19660, t19661, t19668, t19674, t19676, t19677, t19678, t19679)
}
