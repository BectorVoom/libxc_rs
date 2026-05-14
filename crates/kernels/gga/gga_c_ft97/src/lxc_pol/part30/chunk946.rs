//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 946/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk946<F: Float>(t1882: F, t35590: F, t35726: F, t10007: F, t1131: F, t11593: F, t141727: F, t14175: F, t14182: F, t142058: F, t142074: F, t142083: F, t142117: F, t149764: F, t149870: F, t151077: F, t151362: F, t1901: F, t2360: F, t242: F, t2568: F, t2574: F, t28284: F, t33274: F, t33489: F, t35678: F, t3746: F, t3821: F, t3859: F, t3875: F, t3880: F, t3886: F, t446: F, t6079: F, t6154: F, t6947: F, t713: F, t729: F, t7502: F, t7546: F, t7553: F, t762: F) -> (F,) {
    let t152082 = t1882 * t35590;
    let t152087 = t1882 * t35726;
    let t152095 = -2.0 / 3.0 * t446 * t242 * t149764 - t446 * t242 * t149870 / 3.0 - 2.0 / 9.0 * t142058 + t446 * t729 * t762 * t35678 * t713 / 3.0 - t142074 + 4.0 / 9.0 * t11593 * t10007 * t7502 * t3746 - t1901 * t10007 * t141727 * t3875 / 9.0 - 2.0 / 9.0 * t1901 * t14175 * t141727 * t3880 - 2.0 / 9.0 * t1901 * t14182 * t7553 * t2360 * t3886 + 4.0 / 3.0 * t446 * t2574 * t6947 * t6079 + 2.0 / 3.0 * t446 * t729 * t6154 * t28284 - 2.0 / 3.0 * t446 * t729 * t2568 * t7546 * t3821 - 2.0 * t446 * t242 * t151077 + t142083 + t446 * t729 * t33274 * t3859 / 3.0 + t152082 / 27.0 + 4.0 / 3.0 * t446 * t242 * t151362 - 2.0 / 9.0 * t152087 + t446 * t729 * t762 * t33489 * t1131 / 3.0 - t142117 / 9.0;
    (t152095,)
}
