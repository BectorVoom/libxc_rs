//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 959/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk959<F: Float>(t1112: F, t1184: F, t1187: F, t483: F, t1131: F, t2829: F, t1185: F, t2910: F, t2824: F, t465: F, t2825: F, t1175: F, t1738: F) -> (F, F, F, F, F, F) {
    let t10973 = t1184 * t1112 * t483 * t1187;
    let t10976 = t2829 * t1131 * t1187;
    let t10980 = F::new(0.00010931146159029059) * t1185 * t2910 * t1187;
    let t10983 = t2824 * t465 * t483 * t1187;
    let t10987 = F::new(0.0006558687695417436) * t2825 * t1131 * t1187;
    let t10988 = t1738 * t1175;
    (t10973, t10976, t10980, t10983, t10987, t10988)
}
