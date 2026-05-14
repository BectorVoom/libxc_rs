//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1288/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1288<F: Float>(t111167: F, t9365: F, t32592: F, t32664: F, t32647: F, t110930: F, t9379: F, t110925: F, t110934: F, t111153: F, t111156: F, t111159: F, t111162: F, t111168: F, t111173: F, t15722: F, t2701: F) -> (F, F) {
    let t111175 = t9365 * t111167;
    let t111177 = t32664 * t32592;
    let t111179 = t32647 * t32592;
    let t111181 = t9379 * t110930;
    let t111183 = t9379 * t110925;
    let t111185 = t9365 * t110934;
    let t111187 = 0.16296437500000000001e-1 * t111153 + 0.41786499999999999999e-1 * t111156 - 0.55715333333333333331e-1 * t111159 - 0.31250000000000000001e-1 * t111162 + 0.62500000000000000002e-1 * t111168 - 0.62500000000000000002e-1 * t111173 + 0.24125000000000000001e-1 * t111175 - 0.62500000000000000002e-1 * t111177 - 0.62500000000000000002e-1 * t111179 - 0.31250000000000000001e-1 * t111181 - 0.31250000000000000001e-1 * t111183 + 0.36187500000000000001e-1 * t111185;
    let t111193 = t2701 * t15722;
    (t111187, t111193)
}
