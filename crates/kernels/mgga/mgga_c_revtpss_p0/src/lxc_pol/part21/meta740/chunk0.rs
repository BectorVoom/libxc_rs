//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2604/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2604<F: Float>(t13726: F, t9303: F, t13725: F, t1445: F, t2439: F, t14082: F, t3920: F, t14078: F, t2470: F, t3915: F, t13735: F, t2435: F) -> (F, F, F, F, F) {
    let t47938 = t9303 * t13726;
    let t47942 = t2439 * t13725 * t1445;
    let t47944 = t14082 * t3920;
    let t47945 = F::cast_from(0.39029762157531132076e-1_f64) * t47944;
    let t47947 = t3915 * t14078 * t2470;
    let t47948 = F::cast_from(0.39029762157531132076e-1_f64) * t47947;
    let t47952 = t2435 * t13735;
    (t47938, t47942, t47945, t47948, t47952)
}
