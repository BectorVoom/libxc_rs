//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2461/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2461<F: Float>(t1587: F, t2: F, t580: F, t11506: F, t6189: F, t11509: F, t972: F, t981: F, t11144: F, t5819: F, t606: F, t11142: F) -> (F, F, F, F, F, F, F, F) {
    let t18890 = t1587 * t2;
    let t18892 = F::cast_from(2.0_f64) * t18890 * t580;
    let t18898 = t11506 * t6189;
    let t18899 = t11509 * t972;
    let t18900 = t18898 * t18899;
    let t18902 = F::cast_from(0.10254018858216406658e4_f64) * t981 * t18900;
    let t18903 = t11144 * t5819;
    let t18904 = t18903 * t606;
    let t18905 = t11142 * t18904;
    (t18890, t18892, t18898, t18900, t18902, t18903, t18904, t18905)
}
