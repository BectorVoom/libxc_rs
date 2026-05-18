//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 929/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk929<F: Float>(t1185: F, t1187: F, t2910: F, t2824: F, t465: F, t483: F, t1131: F, t2825: F, t1175: F, t1738: F, t1179: F, t10764: F) -> (F, F, F, F, F, F) {
    let t10980 = F::new(0.00010931146159029059) * t1185 * t2910 * t1187;
    let t10983 = t2824 * t465 * t483 * t1187;
    let t10987 = F::new(0.0006558687695417436) * t2825 * t1131 * t1187;
    let t10988 = t1738 * t1175;
    let t10991 = F::new(0.31931290694012293) * t1738 * t1179;
    let t10995 = F::new(0.0012955432484775182) * t10764 * t1187;
    (t10980, t10983, t10987, t10988, t10991, t10995)
}
