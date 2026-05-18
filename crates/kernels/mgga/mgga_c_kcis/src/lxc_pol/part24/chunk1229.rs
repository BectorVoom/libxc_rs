//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1229/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1229<F: Float>(t100023: F, t1142: F, t99903: F, t99943: F, t99983: F, t10498: F, t1203: F, t29042: F, t27987: F, t5189: F, t46041: F, t8064: F) -> (F, F, F, F) {
    let t100026 = t1142 * (t99903 + t99943 + t99983 + t100023);
    let t100029 = F::new(6.0) * t10498 * t29042 * t1203;
    let t100031 = F::new(2.0) * t27987 * t5189;
    let t100033 = F::new(4.0) * t46041 * t8064;
    (t100026, t100029, t100031, t100033)
}
