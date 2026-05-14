//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1045/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1045<F: Float>(t10195: F, t178: F, t915: F, t10050: F, t2380: F, t6475: F, t10063: F, t8273: F, t10116: F, t3174: F, t68: F, t931: F, t9795: F, t10071: F, t3206: F, t926: F) -> (F, F, F, F, F, F) {
    let t26975 = t915 * t10195 * t178;
    let t26981 = t2380 * t6475 * t10050;
    let t26986 = t10063 * t8273;
    let t26995 = t3174 * t68 * t10116;
    let t27001 = t931 * t9795;
    let t27007 = t3206 * t926 * t10071;
    (t26975, t26981, t26986, t26995, t27001, t27007)
}
