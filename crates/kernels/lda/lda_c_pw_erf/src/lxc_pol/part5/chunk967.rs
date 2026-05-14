//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 967/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk967<F: Float>(t156: F, t7914: F, t426: F, t7919: F, t14685: F, t14718: F, t127: F, t14684: F, t14692: F, t14844: F, t14850: F, t1832: F, t1852: F, t2610: F, t5578: F, t6121: F, t7116: F) -> (F, F, F, F, F) {
    let t20427 = t156 * t7914;
    let t20428 = t426 * t20427;
    let t20430 = t156 * t7919;
    let t20431 = t426 * t20430;
    let t20433 = 3.8973666666666666 * t14685;
    let t20434 = 4.5469277777777775 * t14718;
    let t20435 = -88.1424 * t127 * t7116 * t1832 + 17.62848 * t127 * t5578 * t2610 + 17.62848 * t127 * t1852 * t6121 + t20428 / 6.0 + 2.0 * t20431 - t14684 - t20433 + t14692 - t14844 - t14850 + t20434;
    (t20427, t20430, t20433, t20434, t20435)
}
