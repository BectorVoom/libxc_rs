//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1087/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1087<F: Float>(t21926: F, t342: F, t630: F, t10915: F, t1212: F, t1526: F, t15567: F, t17727: F, t17732: F, t17744: F, t17749: F, t17753: F, t17766: F, t17780: F, t18961: F, t18968: F, t18989: F, t18992: F, t19011: F, t19240: F, t231: F, t2320: F, t2917: F, t343: F, t3691: F, t3700: F, t61123: F, t72944: F, t72952: F, t72962: F, t72977: F, t72981: F, t72992: F) -> (F,) {
    let t82552 = t342 * t630 * t21926;
    let t82561 = t72952 / 9.0 - t72962 + t72977 / 9.0 - t72981 - 2.0 / 9.0 * t15567 * t10915 * t1212 * t3691 - t15567 * t18968 * t17744 / 2.0 - 2.0 / 3.0 * t61123 * t18968 * t17780 + t15567 * t2917 * t1212 * t3700 / 3.0 + t15567 * t18968 * t17727 / 6.0 - t15567 * t18961 * t17732 / 9.0 + 2.0 / 3.0 * t15567 * t18961 * t17766 - 7.0 / 27.0 * t15567 * t72944 * t17749 + 4.0 / 9.0 * t61123 * t18961 * t17753 + t72992 / 27.0 + t18989 - t82552 / 12.0 + t18992 - t342 * t343 * t231 * t19240 / 4.0 - t1526 * t2320 * t19011 / 12.0;
    (t82561,)
}
