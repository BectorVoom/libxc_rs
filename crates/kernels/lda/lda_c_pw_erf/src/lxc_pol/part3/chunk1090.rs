//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1090/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1090<F: Float>(t1143: F, t1901: F, t5451: F, t632: F, t1905: F, t2929: F, t781: F, t242: F, t4422: F, t4437: F, t11675: F, t11678: F, t11681: F, t11685: F, t11686: F, t11891: F, t11892: F, t11894: F, t11895: F, t11897: F, t11901: F, t11903: F, t11906: F) -> (F, F, F, F, F, F, F) {
    let t14950 = t1901 * t1143;
    let t14954 = t5451 * t632;
    let t14956 = t1905 * t1143;
    let t14957 = 0.2512884616065132 * t14956;
    let t14958 = t781 * t2929;
    let t14960 = t4422 * t242;
    let t14961 = 0.5025769232130264 * t14960;
    let t14965 = t4437 * t242;
    let t14975 = t11675 + t11678 - t11681 - t11685 + t11686 - t11891 - t11892 - t11894 - t11895 - t11897 - t11901 - t11903 + t11906;
    (t14950, t14954, t14957, t14958, t14961, t14965, t14975)
}
