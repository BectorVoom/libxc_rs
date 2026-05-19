//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 746/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk746<F: Float>(t2425: F, t595: F, t2443: F, t515: F, t3985: F, t3988: F, t3992: F, t5039: F, t5055: F, t5859: F, t6751: F, t6755: F, t6758: F, t6761: F, t6765: F, t6769: F, t6773: F, t6776: F, t6778: F) -> (F, F, F, F) {
    let t6780 = F::new(2.0) / F::new(15.0) * t2425 * t595;
    let t6781 = t2443 * t515;
    let t6782 = F::new(4.0) / F::new(45.0) * t6781;
    let t6784 = -t6751 + t6755 - t6758 - t6761 - t6765 + t6769 - t6773 - F::new(2.0) / F::new(27.0) * t3985 - t3988 + t3992 - t6776 - t6778 - t6780 + t6782 + F::cast_from(0.06649088888888889_f64) * t5859 + t5039 - t5055;
    (t6780, t6781, t6782, t6784)
}
