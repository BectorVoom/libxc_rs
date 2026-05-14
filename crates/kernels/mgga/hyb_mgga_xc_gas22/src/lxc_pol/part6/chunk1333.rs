//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1333/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1333<F: Float>(t11335: F, t9507: F, t15686: F, t9645: F, t9656: F, t1145: F, t2889: F, t4524: F, t2893: F, t11515: F, t14626: F, t26846: F, t26850: F, t31222: F, t31229: F, t31248: F, t31330: F, t7734: F, t7780: F, t9639: F, t9663: F, t9670: F, t9678: F, t9793: F) -> (F, F, F, F, F) {
    let t31347 = t11335 * t9507;
    let t31352 = t15686 * t9645;
    let t31355 = t15686 * t9656;
    let t31363 = t1145 * t4524 * t2889;
    let t31367 = t1145 * t4524 * t2893;
    let t31370 = -6400.0 / 243.0 * t9670 * t31229 + 1408.0 / 243.0 * t9678 * t31330 - 6400.0 / 243.0 * t9663 * t31229 + 704.0 / 81.0 * t9663 * t31222 - 704.0 / 81.0 * t9670 * t31347 - 704.0 / 27.0 * t9639 * t31347 - 2048.0 / 243.0 * t26846 * t31352 + 2048.0 / 243.0 * t26850 * t31355 + 40000.0 / 81.0 * t14626 * t31248 + 1600.0 / 27.0 * t9793 * t11515 + 126.0 * t7734 * t31363 - 168.0 * t7780 * t31367;
    (t31352, t31355, t31363, t31367, t31370)
}
