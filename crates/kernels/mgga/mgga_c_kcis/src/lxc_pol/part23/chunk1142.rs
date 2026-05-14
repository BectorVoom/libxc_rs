//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1142/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1142<F: Float>(t1607: F, t613: F, t17980: F, t27615: F, t1598: F, t18256: F, t251: F, t27567: F, t27583: F, t27598: F, t28805: F, t3801: F, t4440: F, t7981: F, t94223: F, t94225: F, t98014: F, t98022: F, t98030: F, t98036: F, t98039: F, t98994: F) -> (F, F, F) {
    let t99002 = t613 * t1607;
    let t99004 = t99002 * t17980 * t27615;
    let t99013 = t18256 * t251 * t1598;
    let t99016 = -0.23214722222222222222e-2 * t98014 - 0.92835860883789062501e-5 * t98994 * t27598 - 0.61905925925925925924e-2 * t98022 - 0.11607361111111111111e-2 * t98030 - 0.15476481481481481481e-2 * t94223 + 0.10317654320987654321e-2 * t94225 + 0.25794135802469135802e-2 * t98036 + 0.2782641015625e-3 * t27567 * t99004 - 0.15476481481481481481e-2 * t98039 - 0.23168402777777777778e-3 * t27583 * t4440 * t28805 * t3801 - 0.23168402777777777778e-3 * t99013 * t7981;
    (t99004, t99013, t99016)
}
