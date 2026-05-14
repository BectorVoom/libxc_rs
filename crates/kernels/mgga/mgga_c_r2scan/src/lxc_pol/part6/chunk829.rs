//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 829/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk829<F: Float>(t1416: F, t726: F, t2035: F, t424: F, t41: F, t236: F, t4715: F, t735: F, t1422: F, t661: F, t230: F, t4911: F, t1783: F, t60: F, t170: F, t5717: F, t61: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5889 = 60.0 * t1416 * t726;
    let t5890 = t424 * t2035;
    let t5891 = t41 * t5890;
    let t5893 = t4715 * t236;
    let t5895 = 0.16867793133802706421e-1 * t735 * t5893;
    let t5896 = t1422 * t726;
    let t5897 = 96.0 * t5896;
    let t5898 = t1422 * t661;
    let t5901 = 24.0 * t4911 * t230;
    let t5902 = t60 * t1783;
    let t5903 = t5902 * t170;
    let t5907 = 0.11407595979765752406e3 * t61 * t5717;
    (t5889, t5890, t5891, t5893, t5895, t5896, t5897, t5898, t5901, t5902, t5903, t5907)
}
