//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1185;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1186;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta383<F: Float>(t15418: F, t1714: F, t1174: F, t1716: F, t2402: F, t15394: F, t11554: F, t1706: F, t1709: F, t44633: F, t10401: F, t15567: F, t3610: F, t1227: F, t1653: F, t248: F, t45293: F, t11677: F, t15245: F, t10469: F, t1720: F, t10471: F, t11737: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t52059, t52081, t52100, t52124, t52281, t52627) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1185::<F>(t15418, t1714, t1174, t1716, t2402, t15394, t11554, t1706, t1709, t44633, t10401, t15567);
        let (t52628, t52680, t52766, t52834, t52835, t52836) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1186::<F>(t3610, t52627, t1227, t1653, t248, t45293, t11677, t15245, t10469, t1720, t10471, t11737);
    (t52059, t52081, t52100, t52124, t52281, t52627, t52628, t52680, t52766, t52834, t52835, t52836)
}
