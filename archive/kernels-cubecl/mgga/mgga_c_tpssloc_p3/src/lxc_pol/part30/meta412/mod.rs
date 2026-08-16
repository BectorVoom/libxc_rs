//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta412 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1565;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1566;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1567;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1568;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta412<F: Float>(t3247: F, t5398: F, t607: F, t1088: F, t123: F, t1089: F, t16558: F, t11137: F, t11459: F, t14702: F, t14720: F, t14946: F, t14947: F, t18203: F, t18208: F, t18213: F, t18217: F, t18219: F, t18223: F, t18227: F, t18229: F, t18234: F, t423: F, t14858: F, t1703: F, t4869: F, t4879: F, t1117: F, t6021: F, t3264: F, t3315: F, t6020: F, t3313: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t18237, t18239) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1565::<F>(t3247, t5398, t607, t1088, t123);
        let (t18241, t18243) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1566::<F>(t1089, t16558, t1088, t123);
        let t18245 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1567::<F>(t11137, t11459, t14702, t14720, t14946, t14947, t18203, t18208, t18213, t18217, t18219, t18223, t18227, t18229, t18234, t18239, t18243);
        let (t18247, t18249, t18251, t18257, t18261) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1568::<F>(t18245, t423, t14858, t1703, t4869, t4879, t1117, t6021, t3264, t3315, t6020, t3313);
    (t18237, t18239, t18241, t18243, t18247, t18249, t18251, t18257, t18261)
}
