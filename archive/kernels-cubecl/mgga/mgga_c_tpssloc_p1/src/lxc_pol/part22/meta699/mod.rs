//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta699 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2282;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2283;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta699<F: Float>(t1222: F, t18574: F, t11789: F, t1227: F, t248: F, t5975: F, t18321: F, t3548: F, t15437: F, t15502: F, t15506: F, t4965: F, t5023: F, t15643: F, t5024: F, t19201: F, t3576: F, t3577: F, t44951: F, t6191: F, t13969: F, t19061: F, t3515: F, t15568: F, t5064: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t65681, t65689, t65691, t65703, t65706, t65709) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2282::<F>(t1222, t18574, t11789, t1227, t248, t5975, t18321, t3548, t15437, t15502, t15506, t4965, t5023);
        let (t65803, t65815, t65819, t65881, t65884) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2283::<F>(t15643, t5024, t19201, t3576, t3577, t44951, t6191, t13969, t19061, t3515, t15568, t5064);
    (t65681, t65689, t65691, t65703, t65706, t65709, t65803, t65815, t65819, t65881, t65884)
}
