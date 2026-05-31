//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 745/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk745<F: Float>(t1972: F, t6762: F, t3965: F, t5146: F, t784: F, t1967: F, t3967: F, t494: F, t6711: F, t2067: F, t822: F, t2443: F, t544: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6763 = t6762 * t1972;
    let t6765 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t3965 * t6763;
    let t6766 = t5146 * t784;
    let t6767 = t6766 * t1967;
    let t6769 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t3965 * t6767;
    let t6771 = t3967 * t6711 * t494;
    let t6773 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t3965 * t6771;
    let t6776 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t822 * t2067;
    let t6778 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t2443 * t544;
    (t6763, t6765, t6766, t6767, t6769, t6771, t6773, t6776, t6778)
}
