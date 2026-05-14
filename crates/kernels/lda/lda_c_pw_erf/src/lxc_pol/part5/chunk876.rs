//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 876/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk876<F: Float>(t102: F, t436: F, t3296: F, t9: F, t155: F, t1697: F, t5515: F, t925: F, t2061: F, t5518: F, t1652: F, t1833: F, t933: F, t1870: F, t1872: F, t473: F) -> (F, F, F, F, F, F, F) {
    let t14657 = t102 * t436;
    let t14674 = t9 * t3296;
    let t14679 = t155 * t1697;
    let t14683 = t5515 * t925;
    let t14684 = 1.9486833333333333 * t14683;
    let t14685 = t5518 * t2061;
    let t14691 = t1652 * t1833 * t933;
    let t14692 = 0.9743416666666667 * t14691;
    let t14698 = t1870 * t473 * t436 * t1872;
    (t14657, t14674, t14679, t14684, t14685, t14692, t14698)
}
