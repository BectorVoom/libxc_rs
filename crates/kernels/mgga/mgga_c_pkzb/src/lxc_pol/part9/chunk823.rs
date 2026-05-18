//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 823/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk823<F: Float>(t2020: F, t5712: F, t2032: F, t2099: F, t2026: F, t5717: F, t750: F) -> (F, F, F, F) {
    let t5925 = t2020 * t5712;
    let t5928 = t2099 * t2032;
    let t5929 = t2026 * t5928;
    let t5931 = t5717 * t750;
    (t5925, t5928, t5929, t5931)
}
