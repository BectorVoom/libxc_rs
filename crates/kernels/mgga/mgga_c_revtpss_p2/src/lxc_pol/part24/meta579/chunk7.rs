//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1792/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1792<F: Float>(t12787: F, t17693: F, t17694: F, t17729: F, t17747: F, t1785: F, t20956: F, t225: F, t24647: F, t24680: F, t3720: F, t480: F, t484: F, t5046: F, t59144: F, t71718: F, t71744: F, t84029: F, t84032: F, t84061: F, t84645: F, t89883: F, t90881: F) -> F {
    let t91378 = F::cast_from(0.28582678745379824648e-2_f64) * t17693 * t17694 * t90881 - F::cast_from(0.77173232612525526552e-2_f64) * t17747 * t3720 * t20956 * t84645 - F::cast_from(0.28582678745379824648e-2_f64) * t17729 * t12787 * t5046 * t24647 - F::new(154.0) / F::new(243.0) * t84029 - F::new(10.0) / F::new(243.0) * t59144 + F::new(2.0) / F::new(27.0) * t84032 - F::new(2.0) / F::new(81.0) * t71718 - F::cast_from(0.22866142996303859718e-2_f64) * t84061 - F::cast_from(0.96545937095505185475e-2_f64) * t71744 - F::cast_from(0.21240106161011140804e0_f64) * t1785 * t24680 * t484 + F::cast_from(0.21437009059034868486e-3_f64) * t89883 * t225 * t480 * t484;
    t91378
}
