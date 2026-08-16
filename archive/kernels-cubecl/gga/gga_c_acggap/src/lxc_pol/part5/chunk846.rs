//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 846/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk846<F: Float>(t243: F, t2824: F, t40: F, t803: F, t901: F, t685: F, t790: F, t687: F, t2795: F, t286: F, t244: F, t2974: F) -> (F, F, F, F, F, F) {
    let t11849 = t40 * t243 * t2824;
    let t11856 = t40 * t901 * t803;
    let t11869 = F::cast_from(1.0_f64) / t685 / t790;
    let t11870 = t687 * t687;
    let t11874 = F::cast_from(0.12304822629859687989e5_f64) * t286 * t11869 * t11870 * t2795;
    let t11878 = t2974 * t244;
    (t11849, t11856, t11869, t11870, t11874, t11878)
}
