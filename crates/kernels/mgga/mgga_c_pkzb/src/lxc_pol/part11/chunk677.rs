//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 677/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk677<F: Float>(t1597: F, t470: F, t57: F, t1517: F, t490: F, t1600: F, t74: F) -> (F, F, F, F, F, F) {
    let t4998 = 1.0 / t1597 / t470;
    let t4999 = t57 * t4998;
    let t5000 = t1517 * t490;
    let t5002 = 1.0 / t1600 / t74;
    let t5003 = t5000 * t5002;
    let t5005 = 0.51726012919273400301e3 * t4999 * t5003;
    (t4998, t4999, t5000, t5002, t5003, t5005)
}
