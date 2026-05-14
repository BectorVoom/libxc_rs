//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 690/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk690<F: Float>(t3965: F, t6767: F, t3967: F, t494: F, t6711: F, t2067: F, t822: F, t2443: F, t544: F, t2425: F, t595: F, t515: F, t3985: F, t3988: F, t3992: F, t5039: F, t5055: F, t5859: F, t6751: F, t6755: F, t6758: F, t6761: F, t6765: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6769 = 16.0 / 27.0 * t3965 * t6767;
    let t6771 = t3967 * t6711 * t494;
    let t6773 = 16.0 / 45.0 * t3965 * t6771;
    let t6776 = 4.0 / 15.0 * t822 * t2067;
    let t6778 = 2.0 / 15.0 * t2443 * t544;
    let t6780 = 2.0 / 15.0 * t2425 * t595;
    let t6781 = t2443 * t515;
    let t6782 = 4.0 / 45.0 * t6781;
    let t6784 = -t6751 + t6755 - t6758 - t6761 - t6765 + t6769 - t6773 - 2.0 / 27.0 * t3985 - t3988 + t3992 - t6776 - t6778 - t6780 + t6782 + 0.06649088888888889 * t5859 + t5039 - t5055;
    (t6769, t6771, t6773, t6776, t6778, t6780, t6781, t6782, t6784)
}
