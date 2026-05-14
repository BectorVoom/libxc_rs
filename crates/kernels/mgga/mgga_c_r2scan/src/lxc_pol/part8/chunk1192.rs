//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1192/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1192<F: Float>(t4720: F, t4963: F, t4966: F, t4971: F, t4974: F, t4978: F, t4790: F, t4793: F, t4805: F, t4991: F, t4995: F, t4999: F, t5003: F, t5007: F, t4826: F, t4838: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23214 = 0.13780319445925925925e-1 * t4720;
    let t23215 = 0.1301229756036208781e0 * t4963;
    let t23216 = 0.19263893255070628431e1 * t4966;
    let t23218 = 0.65061487801810439052e-1 * t4971;
    let t23219 = 0.86748650402413918736e-1 * t4974;
    let t23221 = 48.0 * t4978;
    let t23225 = 0.28493333333333333333e0 * t4790;
    let t23226 = 0.2137e0 * t4793;
    let t23230 = 0.4274e0 * t4805;
    let t23232 = 0.14035736694323150897e2 * t4991;
    let t23235 = 0.14035736694323150897e2 * t4995;
    let t23236 = 0.20779030926817756511e3 * t4999;
    let t23237 = 0.23392894490538584828e1 * t5003;
    let t23238 = 0.4101607543286562663e4 * t5007;
    let t23240 = 0.3859675079686208416e3 * t4826;
    let t23241 = 4.0 * t4838;
    (t23214, t23215, t23216, t23218, t23219, t23221, t23225, t23226, t23230, t23232, t23235, t23236, t23237, t23238, t23240, t23241)
}
