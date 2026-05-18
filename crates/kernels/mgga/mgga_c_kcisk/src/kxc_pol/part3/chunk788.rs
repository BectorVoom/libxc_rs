//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 788/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk788<F: Float>(t4998: F, t5493: F, t2013: F, t10441: F, t5486: F, t1775: F, t10463: F, t786: F, t10832: F, t5498: F, t10879: F, t2015: F) -> (F, F, F, F, F) {
    let t12162 = t4998 * t5493;
    let t12163 = t2013 * t12162;
    let t12165 = t5486 * t10441;
    let t12166 = t1775 * t12165;
    let t12169 = t786 * t10463;
    let t12170 = t12169 * t10441;
    let t12171 = t10832 * t12170;
    let t12174 = t4998 * t5498;
    let t12175 = t2013 * t12174;
    let t12179 = t10879 * t2015;
    (t12163, t12166, t12171, t12175, t12179)
}
