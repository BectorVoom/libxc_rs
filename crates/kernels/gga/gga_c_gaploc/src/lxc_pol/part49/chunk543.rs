//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 543/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk543<F: Float>(t701: F, t9729: F, t1445: F, t2194: F, t3308: F, t9734: F, t2530: F, t2571: F, t9604: F, t9591: F, t7068: F, t883: F, t1967: F, t7810: F, t3209: F, t5241: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9869 = t9729 * t701;
    let t9870 = t1445 * t9869;
    let t9873 = t2194 * t3308;
    let t9875 = t9734 * t701;
    let t9876 = t1445 * t9875;
    let t9879 = t2571 * t2530;
    let t9880 = t1445 * t9879;
    let t9883 = t1445 * t9604;
    let t9886 = t1445 * t9591;
    let t9889 = t883 * t7068;
    let t9890 = t1967 * t9889;
    let t9891 = t7810 * t9890;
    let t9892 = 0.38342925953920749676e0 * t9891;
    let t9893 = t5241 * t3209;
    (t9870, t9873, t9876, t9880, t9883, t9886, t9889, t9891, t9892, t9893)
}
