//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 841/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk841<F: Float>(t16779: F, t16883: F, t241: F, t1000: F, t16640: F, t914: F, t16632: F, t2549: F, t16648: F, t13603: F, t13607: F, t13612: F, t16654: F, t16657: F, t4054: F, t5069: F, t5076: F, t999: F) -> (F, F, F, F, F, F, F, F) {
    let t16885 = t241 * (t16779 + t16883);
    let t16886 = t1000 * t16640;
    let t16887 = t914 * t16886;
    let t16890 = t2549 * t16632;
    let t16891 = t914 * t16890;
    let t16894 = t1000 * t16648;
    let t16895 = t914 * t16894;
    let t16900 = -t16654 - t16657 - t13603 / 3.0 + t13607 / 3.0 + t13612 / 6.0 + t16885 + t999 * t16887 - t4054 * t5076 - 4.0 / 3.0 * t999 * t16891 + t999 * t16895 / 6.0 + 2.0 / 3.0 * t4054 * t5069;
    (t16885, t16886, t16887, t16890, t16891, t16894, t16895, t16900)
}
