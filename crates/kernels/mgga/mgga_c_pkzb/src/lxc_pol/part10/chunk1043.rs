//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1043/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1043<F: Float>(t5189: F, t7047: F, t3426: F, t496: F, t501: F, t5325: F, t5339: F, t5025: F, t5028: F, t5040: F, t5066: F, t5069: F, t5073: F, t5324: F, t5333: F, t5338: F, t5344: F) -> (F, F, F, F, F, F, F) {
    let t8848 = 20.0 * t5189;
    let t8849 = 0.21687162600603479684e-1 * t7047;
    let t8850 = t496 * t3426;
    let t8851 = 4.0 * t8850;
    let t8852 = t501 * t3426;
    let t8853 = 4.0 * t8852;
    let t8854 = 0.24415263074675393405e-3 * t5325;
    let t8855 = 0.5848223622634646207e0 * t5339;
    let t8856 = t8848 + t5025 + t8849 + t5028 + t8851 - t8853 - t5324 + t5040 + t5066 - t5069 - t5073 + t8854 + t5333 - t5338 - t8855 - t5344;
    (t8848, t8849, t8851, t8853, t8854, t8855, t8856)
}
