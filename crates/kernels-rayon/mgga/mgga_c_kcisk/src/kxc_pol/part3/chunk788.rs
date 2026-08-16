//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 788/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk788(t4998: f64, t5493: f64, t2013: f64, t10441: f64, t5486: f64, t1775: f64, t10463: f64, t786: f64, t10832: f64, t5498: f64, t10879: f64, t2015: f64) -> (f64, f64, f64, f64, f64) {
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
