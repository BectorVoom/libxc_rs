//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 580/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk580<F: Float>(t4292: F, t116: F, t717: F, t732: F, t731: F, t1184: F, t1753: F, t279: F, t1752: F, t1746: F, t1759: F, t2953: F, t739: F, t34: F, t939: F, t2966: F, t743: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4293 = 1.898172889849454 * t4292;
    let t4295 = t732 * t717 * t116;
    let t4296 = t731 * t4295;
    let t4299 = t1753 * t1184 * t279;
    let t4300 = t1752 * t4299;
    let t4304 = t1759 * t1746;
    let t4305 = 2.0538164420033334 * t4304;
    let t4352 = t2953 * t739;
    let t4355 = t939 * t34;
    let t4367 = t2966 * t743;
    (t4293, t4295, t4296, t4299, t4300, t4305, t4352, t4355, t4367)
}
