//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 766/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk766<F: Float>(t5875: F, t5904: F, t5903: F, t492: F, t570: F, t41: F, t4134: F, t4293: F, t5671: F, t4292: F, t5880: F, t4261: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5905 = t5904 * t5875;
    let t5906 = t5903 * t5905;
    let t5908 = t570 * t492;
    let t5909 = t41 * t4134;
    let t5910 = t5909 * t5875;
    let t5911 = t5908 * t5910;
    let t5913 = t4293 * t5671;
    let t5914 = t4292 * t5913;
    let t5916 = t4293 * t5880;
    let t5917 = t4292 * t5916;
    let t5919 = t4261 * t5671;
    (t5905, t5906, t5908, t5909, t5910, t5911, t5913, t5914, t5916, t5917, t5919)
}
