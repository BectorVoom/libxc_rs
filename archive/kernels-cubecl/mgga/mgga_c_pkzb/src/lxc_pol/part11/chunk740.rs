//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 740/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk740<F: Float>(t1939: F, t247: F, t1915: F, t690: F, t1954: F, t709: F, t2020: F, t5712: F, t5717: F, t750: F) -> (F, F, F, F, F) {
    let t5873 = F::cast_from(1.0_f64) / t1939 / t247;
    let t5897 = t690 * t1915;
    let t5903 = t709 * t1954;
    let t5925 = t2020 * t5712;
    let t5931 = t5717 * t750;
    (t5873, t5897, t5903, t5925, t5931)
}
