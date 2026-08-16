//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3534/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3534<F: Float>(t20020: F, t3211: F, t15656: F, t4845: F, t19675: F, t372: F, t11774: F, t11779: F, t11933: F, t15703: F, t15745: F, t16067: F, t16068: F, t1665: F, t20091: F, t3096: F, t3117: F, t4854: F, t54699: F, t54907: F, t54914: F, t54919: F, t54925: F, t6278: F, t65144: F, t66542: F) -> F {
    let t67044 = t3211 * t20020;
    let t67048 = t15656 * t4845;
    let t67052 = t372 * t19675;
    let t67058 = F::cast_from(0.21437009059034868486e-3_f64) * t16067 * t3117 * t65144 * t16068 - F::cast_from(0.19055119163586549765e-3_f64) * t54907 + F::cast_from(0.45732285992607719436e-2_f64) * t11933 * t20091 - F::cast_from(0.57165357490759649296e-3_f64) * t54914 - F::cast_from(0.57165357490759649296e-3_f64) * t54919 - F::cast_from(0.7622047665434619906e-3_f64) * t54925 - F::cast_from(0.72409452821628889107e-2_f64) * t11779 * t6278 + F::cast_from(0.15244095330869239812e-2_f64) * t67044 + F::cast_from(0.45732285992607719436e-2_f64) * t54699 * t1665 - F::cast_from(0.57165357490759649296e-3_f64) * t67048 + F::cast_from(0.45732285992607719436e-2_f64) * t15745 * t4854 - F::cast_from(0.28582678745379824648e-3_f64) * t11774 * t67052 * t3096 - F::cast_from(0.11433071498151929859e-2_f64) * t66542 * t15703;
    t67058
}
