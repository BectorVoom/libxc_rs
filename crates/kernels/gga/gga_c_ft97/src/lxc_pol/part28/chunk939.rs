//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 939/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk939<F: Float>(t145790: F, t145807: F, t145824: F, t145840: F, t145859: F, t145875: F, t145893: F, t145906: F, t1825: F, t34511: F, t34682: F, t8392: F, t34678: F, t102689: F, t102776: F, t103073: F, t103626: F, t11490: F, t11593: F, t11810: F, t11854: F, t11863: F, t137797: F, t137804: F, t144809: F, t144846: F, t1901: F, t1909: F, t23327: F, t23339: F, t25919: F, t26162: F, t26171: F, t26357: F, t26367: F, t26382: F, t26390: F, t3052: F, t3219: F, t32515: F, t32597: F, t32606: F, t32635: F, t3266: F, t3271: F, t34568: F, t34627: F, t379: F, t46874: F, t47443: F, t59631: F, t7229: F, t8557: F) -> (F, F, F) {
    let t145909 = t145790 + t145807 + t145824 + t145840 + t145859 + t145875 + t145893 + t145906;
    let t145922 = t1825 * t34511;
    let t145931 = t8392 * t34682;
    let t145964 = t8392 * t34678;
    let t145991 = -2.0 / 9.0 * t1901 * t11863 * t144809 - t1901 * t8557 * t34627 * t379 / 9.0 + 2.0 / 27.0 * t145931 + 2.0 / 3.0 * t1901 * t46874 * t144846 - 2.0 / 3.0 * t1901 * t11810 * t32515 * t3266 - 2.0 / 3.0 * t1901 * t11490 * t137797 * t3271 + 2.0 / 9.0 * t1901 * t23327 * t26357 - 4.0 / 3.0 * t1901 * t59631 * t32606 - 2.0 * t1901 * t26171 * t32597 * t3266 - 2.0 / 3.0 * t1901 * t11810 * t137804 * t3271 - 4.0 / 3.0 * t1901 * t103073 * t26162 - 4.0 / 3.0 * t1901 * t102776 * t26382 + 2.0 / 27.0 * t145964 + 4.0 * t1901 * t103626 * t7229 * t3219 - 2.0 / 9.0 * t1901 * t11854 * t34568 * t379 - 2.0 / 9.0 * t1901 * t47443 * t32635 - 4.0 / 9.0 * t1901 * t102689 * t25919 + 2.0 / 9.0 * t11593 * t1909 * t32515 * t3052 - 4.0 / 3.0 * t1901 * t102776 * t26367 - 4.0 / 3.0 * t1901 * t11810 * t23339 * t26390;
    (t145909, t145922, t145991)
}
