//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 925/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk925<F: Float>(t21643: F, t3201: F, t3188: F, t5866: F, t1629: F, t1058: F, t11046: F, t11059: F, t11065: F, t14608: F, t14618: F, t1610: F, t1630: F, t1632: F, t18086: F, t21481: F, t21615: F, t21618: F, t21623: F, t21627: F, t21635: F, t21638: F, t21644: F, t21647: F, t21650: F, t3186: F, t3200: F, t353: F, t384: F, t4669: F, t5903: F, t5929: F, t5933: F, t5937: F, t5939: F, t5941: F) -> F {
    let t21653 = t21643 * t3201;
    let t21656 = t3188 * t5866;
    let t21657 = t1629 * t21656;
    let t21662 = F::cast_from(3.0_f64) * t18086 * t1630 + F::cast_from(6.0_f64) * t14618 * t5929 - F::cast_from(3.0_f64) * t14608 * t5939 + t353 * t21615 + F::cast_from(3.0_f64) * t1058 * t21618 - F::cast_from(3.0_f64) * t3200 * t21623 + F::cast_from(3.0_f64) * t1058 * t21627 + F::cast_from(3.0_f64) * t1610 * t5941 + F::cast_from(3.0_f64) * t5903 * t1632 + t1058 * t21635 + t11046 * t21638 + t21481 * t384 + F::cast_from(6.0_f64) * t4669 * t5933 + F::cast_from(6.0_f64) * t3186 * t21644 + F::cast_from(6.0_f64) * t11059 * t21647 - F::cast_from(6.0_f64) * t11065 * t21650 - F::cast_from(3.0_f64) * t3200 * t21653 + F::cast_from(6.0_f64) * t3186 * t21657 + F::cast_from(3.0_f64) * t4669 * t5937;
    t21662
}
