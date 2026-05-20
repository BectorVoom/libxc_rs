//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3767/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3767<F: Float>(t17729: F, t20922: F, t44425: F, t17396: F, t17617: F, t1222: F, t6658: F, t697: F, t6662: F, t12268: F, t12787: F, t15936: F, t17580: F, t17625: F, t17730: F, t1774: F, t20317: F, t3626: F, t5354: F, t56953: F, t57147: F, t59162: F, t59338: F, t59349: F, t59351: F, t59353: F) -> F {
    let t71908 = t17729 * t44425 * t20922;
    let t71920 = t17396 * t17617;
    let t71928 = t1222 * t697 * t6658;
    let t71931 = t1222 * t697 * t6662;
    let t71936 = -F::cast_from(0.19055119163586549765e-3_f64) * t59338 - F::cast_from(0.6351706387862183255e-3_f64) * t71908 - F::cast_from(0.85748036236139473944e-3_f64) * t59162 * t17580 + F::cast_from(0.57165357490759649296e-3_f64) * t17729 * t3626 * t20317 * t17730 + F::cast_from(0.45732285992607719436e-2_f64) * t56953 * t5354 - F::cast_from(0.45732285992607719436e-2_f64) * t57147 * t17625 + F::cast_from(0.30488190661738479624e-2_f64) * t71920 - F::cast_from(0.28582678745379824648e-2_f64) * t17729 * t12787 * t1774 * t12268 * t15936 + t71928 / F::new(1296.0) + t71931 / F::new(648.0) - F::cast_from(0.28582678745379824648e-3_f64) * t59349 - F::cast_from(0.28582678745379824648e-3_f64) * t59351 + F::cast_from(0.57165357490759649296e-3_f64) * t59353;
    t71936
}
