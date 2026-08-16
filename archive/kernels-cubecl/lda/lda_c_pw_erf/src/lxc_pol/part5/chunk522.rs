//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 522/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk522<F: Float>(t2610: F, t436: F, t127: F, t1655: F, t1663: F, t1674: F, t1689: F, t1695: F, t1838: F, t1850: F, t2598: F, t2601: F, t2613: F, t2616: F, t2620: F, t2624: F, t426: F) -> (F, F) {
    let t2627 = t436 * t2610;
    let t2630 = -t1655 + t2598 + t1663 + t2601 - t2613 + t1674 + t1838 / F::cast_from(3.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t426 * t2616 - t426 * t2620 / F::cast_from(2.0_f64) + t1689 + F::cast_from(1.46904_f64) * t1850 + t1695 + F::cast_from(5.87616_f64) * t127 * t2624 - F::cast_from(1.46904_f64) * t127 * t2627;
    (t2627, t2630)
}
