//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1186/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1186<F: Float>(t5634: F, t7706: F, t9461: F, t3759: F, t8054: F, t9452: F, t6204: F, t7740: F, t9447: F, t1312: F, t32107: F, t33373: F, t33384: F, t33452: F, t33460: F, t33463: F, t34693: F, t34697: F, t9426: F, t9446: F, t9796: F, t9805: F, t9809: F) -> (F, F, F, F, F, F, F, F) {
    let t34700 = t5634 * t7706;
    let t34701 = t9461 * t34700;
    let t34702 = t3759 * t34701;
    let t34706 = t9452 * t8054;
    let t34707 = t6204 * t34706;
    let t34714 = t9447 * t7740;
    let t34715 = t1312 * t34714;
    let t34722 = -0.69444444444444444446e-2 * t33373 * t9805 - 0.120625e-1 * t9426 * t34693 + 0.10416666666666666667e-1 * t9446 * t34697 + 0.27636574074074074073e-2 * t34702 + 0.20833333333333333334e-1 * t33384 * t9809 + 0.10416666666666666667e-1 * t9446 * t34707 + 0.8041666666666666667e-2 * t33460 * t9796 + 0.20833333333333333334e-1 * t33384 * t9796 + 0.69444444444444444446e-2 * t9446 * t34715 + t32107 + 0.69444444444444444446e-2 * t33452 - 0.20833333333333333334e-1 * t9446 * t34693 + 0.26805555555555555556e-2 * t33463;
    (t34700, t34701, t34702, t34706, t34707, t34714, t34715, t34722)
}
