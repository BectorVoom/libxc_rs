//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1029/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1029<F: Float>(t122277: F, t25898: F, t25901: F, t25953: F, t32716: F, t2022: F, t28911: F, t32729: F, t121045: F, t122273: F, t26050: F, t122295: F, t32275: F, t94382: F, t1955: F, t32689: F, t4075: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t122335 = t122277 * t25898;
    let t122336 = t122335 * t25901;
    let t122341 = 0.34270468708064099208e-1 * t32716 * t25953;
    let t122346 = t28911 * t2022;
    let t122351 = 0.19274729307122665472e-1 * t32729 * t25953;
    let t122355 = 0.98339826130601561944e-2 * t121045;
    let t122357 = t122273 * t25898;
    let t122358 = t122357 * t25901;
    let t122391 = t32729 * t26050;
    let t122393 = t32716 * t26050;
    let t122399 = 0.95199562775170587692e-3 * t94382 * t32275 * t122295;
    let t122407 = t1955 * t32689 * t4075;
    (t122335, t122336, t122341, t122346, t122351, t122355, t122357, t122358, t122391, t122393, t122399, t122407)
}
