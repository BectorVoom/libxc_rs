//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2191;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta532<F: Float>(t18080: F, t3201: F, t3188: F, t1057: F, t18028: F, t1615: F, t4657: F, t1060: F, t1022: F, t360: F, t6739: F, t5928: F) -> (F, F, F, F, F, F, F) {
        let (t18081, t18083, t18086, t18088, t18089, t18093, t18094) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2191::<F>(t18080, t3201, t3188, t1057, t18028, t1615, t4657, t1060, t1022, t360, t6739, t5928);
    (t18081, t18083, t18086, t18088, t18089, t18093, t18094)
}
