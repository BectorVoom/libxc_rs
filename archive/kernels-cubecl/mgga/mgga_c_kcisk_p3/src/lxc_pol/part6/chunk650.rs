//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 650/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk650<F: Float>(t719: F, t8939: F, t735: F, t1935: F, t2564: F, t7337: F, t642: F, t8786: F, t734: F, t2576: F, t2587: F, t2591: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9029 = t719 * t8939;
    let t9030 = t735 * t9029;
    let t9031 = t1935 * t9030;
    let t9033 = t7337 * t2564;
    let t9035 = t642 * t8786;
    let t9036 = t735 * t9035;
    let t9037 = t734 * t9036;
    let t9039 = t2576 * t2587;
    let t9041 = t2576 * t2591;
    (t9029, t9030, t9031, t9033, t9035, t9036, t9037, t9039, t9041)
}
