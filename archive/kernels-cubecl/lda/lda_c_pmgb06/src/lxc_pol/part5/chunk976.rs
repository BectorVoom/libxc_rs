//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 976/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk976<F: Float>(t4948: F, t831: F, t486: F, t6616: F, t132: F, t1547: F, t2583: F, t2470: F, t3223: F, t1447: F, t6120: F, t2477: F, t3213: F) -> (F, F, F, F, F, F) {
    let t16294 = t831 * t4948;
    let t16298 = t486 * t6616;
    let t16305 = t132 * t1547 * t2583;
    let t16307 = t3223 * t2470;
    let t16309 = t1447 * t6120;
    let t16314 = t3213 * t2477;
    (t16294, t16298, t16305, t16307, t16309, t16314)
}
