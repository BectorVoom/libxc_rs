//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2269;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta563<F: Float>(t1226: F, t6169: F, t486: F, t6218: F, t4978: F, t4582: F, t1216: F, t17635: F, t4987: F, t4977: F, t5012: F, t11836: F, t1218: F, t1227: F, t1232: F, t15495: F, t15727: F, t15731: F, t15735: F, t15745: F, t1737: F, t19033: F, t19041: F, t19047: F, t3506: F, t3515: F, t3536: F, t4989: F, t5024: F, t6221: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19051, t19056, t19057, t19058, t19061, t19062, t19067, t19068, t19071, t19072, t19075) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2269::<F>(t1226, t6169, t486, t6218, t4978, t4582, t1216, t17635, t4987, t4977, t5012, t11836, t1218, t1227, t1232, t15495, t15727, t15731, t15735, t15745, t1737, t19033, t19041, t19047, t3506, t3515, t3536, t4989, t5024, t6221);
    (t19051, t19056, t19057, t19058, t19061, t19062, t19067, t19068, t19071, t19072, t19075)
}
