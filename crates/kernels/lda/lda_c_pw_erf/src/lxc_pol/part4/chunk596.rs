//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 596/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk596<F: Float>(t147: F, t2824: F, t483: F, t1187: F, t1184: F, t465: F, t1131: F, t1185: F, t1171: F, t684: F, t1175: F, t1738: F, t692: F, t688: F, t1179: F, t34: F, t39: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2825 = t2824 * t147;
    let t2826 = t2825 * t483;
    let t2828 = 0.0001639671923854359 * t2826 * t1187;
    let t2829 = t1184 * t465;
    let t2830 = t2829 * t483;
    let t2831 = t2830 * t1187;
    let t2833 = t1185 * t1131;
    let t2835 = 5.4655730795145296e-05 * t2833 * t1187;
    let t2836 = t684 * t1171;
    let t2838 = t684 * t1175;
    let t2841 = 0.15965645347006147 * t1738 * t692;
    let t2842 = t1738 * t688;
    let t2847 = 0.05987117005127304 * t684 * t1179;
    let t2849 = t34 * t39;
    (t2825, t2826, t2828, t2829, t2830, t2831, t2833, t2835, t2836, t2838, t2841, t2842, t2847, t2849)
}
