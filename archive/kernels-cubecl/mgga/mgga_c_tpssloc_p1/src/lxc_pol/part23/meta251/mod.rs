//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta251 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk910;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta251<F: Float>(t3400: F, t6084: F, t300: F, t6063: F, t4997: F, t5002: F, t11784: F, t248: F, t5971: F, t1227: F, t5019: F, t4993: F, t5005: F, t5024: F, t1017: F, t6163: F, t1210: F, t1207: F, t372: F, t479: F, t471: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18910, t18915, t18972, t18975, t18976, t18978, t18980) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk910::<F>(t3400, t6084, t300, t6063, t4997, t5002, t11784, t248, t5971, t1227, t5019, t4993, t5005);
        let (t18987, t19025, t19026, t19032, t19033) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk911::<F>(t4993, t5024, t1017, t6163, t1210, t1207, t372, t479, t471);
    (t18910, t18915, t18972, t18975, t18976, t18978, t18980, t18987, t19025, t19026, t19032, t19033)
}
