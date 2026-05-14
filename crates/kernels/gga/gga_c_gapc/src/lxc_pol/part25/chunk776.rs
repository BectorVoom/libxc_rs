//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 776/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk776<F: Float>(t3345: F, t9763: F, t3424: F, t3427: F, t3431: F, t7675: F, t3437: F, t8915: F, t3440: F, t9731: F, t9734: F, t9742: F, t9745: F, t9748: F, t9751: F, t9754: F, t9758: F, t9761: F) -> (F, F, F, F, F, F) {
    let t9764 = t9763 * t3345;
    let t9766 = t3424 * t3427;
    let t9768 = t7675 * t3431;
    let t9770 = t3437 * t8915;
    let t9771 = t9770 * t3440;
    let t9773 = 0.17376185052903442709e-3 * t9731 - 0.19323635647535681159e-7 * t9734 + 0.2152351096824363426e-6 * t9742 + 0.86880925264517213544e-4 * t9745 - 0.69504740211613770836e-4 * t9748 - 0.69504740211613770836e-4 * t9751 + 0.2085142206348413125e-3 * t9754 + 0.50027140879067581468e-9 * t9758 + 0.77294542590142724635e-6 * t9761 - 0.25745714186718600948e-5 * t9764 - 0.17376185052903442709e-3 * t9766 + 0.14480154210752868924e-5 * t9768 - 0.22509399720615334744e-7 * t9771;
    (t9764, t9766, t9768, t9770, t9771, t9773)
}
