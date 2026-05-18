//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1281/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1281<F: Float>(t7898: F, t98524: F, t27410: F, t28426: F, t3245: F, t8176: F, t27345: F, t8144: F, t1014: F, t28409: F, t27342: F, t27396: F, t27416: F, t28397: F, t8151: F, t94664: F, t94669: F, t98600: F) -> (F, F, F) {
    let t98934 = t7898 * t98524;
    let t98938 = t27410 * t28426;
    let t98942 = t3245 * t8176;
    let t98945 = F::new(0.46336805555555555556e-3) * t8144 * t27345;
    let t98946 = t1014 * t28409;
    let t98950 = F::new(0.16581944444444444444e-2) * t94664 - F::new(0.11054629629629629629e-2) * t94669 + F::new(0.92754700520833333333e-4) * t7898 * t98600 - F::new(0.20612155671296296296e-4) * t98934 + F::new(0.92754700520833333333e-4) * t28397 * t27416 + F::new(0.61836467013888888888e-4) * t98938 - F::new(0.13901041666666666667e-2) * t8144 * t27342 + F::new(0.14739506172839506172e-2) * t98942 + t98945 - F::new(0.5895802469135802469e-2) * t98946 + F::new(0.37069444444444444444e-2) * t8151 * t27396;
    (t98942, t98946, t98950)
}
