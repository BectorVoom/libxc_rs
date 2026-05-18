//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 959/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk959<F: Float>(t3426: F, t395: F, t3430: F, t10762: F, t571: F, t11: F, t10785: F, t10789: F, t10793: F, t10797: F, t10801: F, t10804: F, t10807: F, t10810: F, t10813: F, t10816: F, t10819: F, t10823: F, t25: F, t2718: F, t7407: F, t7409: F) -> (F, F, F, F) {
    let t10825 = t395 * t3426;
    let t10827 = t395 * t3430;
    let t10829 = t571 * t10762;
    let t10830 = t11 * t10829;
    let t10832 = -F::new(0.29629629629629629629e-2) * t25 * t10785 - F::new(0.88888888888888888888e-2) * t2718 * t10789 - F::new(0.39999999999999999999e-1) * t25 * t10793 + F::new(0.53333333333333333332e-1) * t2718 * t10797 - F::new(0.39990740740740740742e-1) * t10801 + F::new(0.14396666666666666667e0) * t10804 - F::new(0.9597777777777777778e-1) * t10807 - F::new(0.21595e0) * t10810 + F::new(0.28793333333333333334e0) * t10813 - F::new(0.23994444444444444445e-1) * t10816 + F::new(0.71983333333333333334e-1) * t10819 - F::new(0.14814814814814814815e-1) * t7407 + F::new(0.17777777777777777778e-1) * t7409 + F::new(0.79981481481481481483e-2) * t10823 - F::new(0.23994444444444444445e-1) * t10825 + F::new(0.11997222222222222222e-1) * t10827 - F::new(0.35991666666666666667e-1) * t10830;
    (t10825, t10827, t10830, t10832)
}
