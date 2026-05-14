//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1241/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1241<F: Float>(t26442: F, t8392: F, t26437: F, t1851: F, t6524: F, t1853: F, t100166: F, t10961: F, t11437: F, t11472: F, t11490: F, t11593: F, t11600: F, t11605: F, t12041: F, t1871: F, t1901: F, t22975: F, t23323: F, t26042: F, t26166: F, t26268: F, t26349: F, t26440: F, t3052: F, t3281: F, t379: F, t446: F, t447: F, t5750: F, t83: F, t8506: F, t91682: F, t91684: F, t986: F) -> (F, F) {
    let t102903 = 4.0 / 81.0 * t8392 * t26442;
    let t102917 = 4.0 / 27.0 * t8392 * t26437;
    let t102921 = t6524 * t1851;
    let t102922 = t102921 * t1853;
    let t102938 = 2.0 / 3.0 * t446 * t1871 * t986 * t22975 + 2.0 / 3.0 * t91682 + 2.0 / 9.0 * t91684 + 2.0 / 3.0 * t1901 * t11472 * t26440 * t11437 - t102903 + 2.0 / 9.0 * t1901 * t8506 * t26268 + 4.0 / 9.0 * t11593 * t23323 * t11600 + 8.0 / 9.0 * t11593 * t23323 * t11605 - 8.0 / 27.0 * t11593 * t26349 * t12041 + t102917 - t446 * t83 * t100166 / 3.0 + 2.0 / 3.0 * t446 * t83 * t102922 - 4.0 / 9.0 * t3281 * t447 * t5750 * t3052 - 2.0 / 9.0 * t446 * t447 * t26042 * t379 - 4.0 / 3.0 * t1901 * t11490 * t26166 * t10961;
    (t102922, t102938)
}
