//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1270/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1270<F: Float>(t185: F, t186: F, t22664: F, t22700: F, t22745: F, t22806: F, t530: F, t18404: F, t18407: F, t18409: F, t18413: F, t22616: F, t22619: F, t22622: F, t22624: F, t22626: F, t22629: F, t22631: F, t22634: F, t22636: F) -> (F, F, F, F, F, F) {
    let t22812 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t185 * t186 * t530 * (t22664 + t22700 + t22745 + t22806);
    let t22813 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t18404;
    let t22814 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t18407;
    let t22815 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t18409;
    let t22816 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t18413;
    let t22817 = -t22616 - t22619 - t22622 - t22624 - t22626 - t22629 - t22631 - t22634 - t22636 - t22812 - t22813 - t22814 - t22815 + t22816;
    (t22812, t22813, t22814, t22815, t22816, t22817)
}
