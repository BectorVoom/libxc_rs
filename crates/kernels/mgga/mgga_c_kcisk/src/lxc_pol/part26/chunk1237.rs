//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1237/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1237<F: Float>(t1101: F, t267: F, t2697: F, t5821: F, t32597: F, t3368: F, t1110: F, t3042: F, t140: F, t3073: F, t9331: F, t3069: F, t3128: F, t2694: F, t43683: F, t32592: F, t32652: F) -> (F, F, F, F, F, F, F, F) {
    let t110962 = t1101 * t267 * t5821 * t2697;
    let t110965 = t3368 * t32597 * t2697;
    let t110969 = t1101 * t1110 * t3042 * t2697;
    let t110972 = t140 * t3073 * t9331;
    let t110975 = t140 * t3069 * t9331;
    let t110978 = t140 * t3128 * t9331;
    let t110981 = t43683 * t2694 * t2697;
    let t110983 = t32652 * t32592;
    (t110962, t110965, t110969, t110972, t110975, t110978, t110981, t110983)
}
