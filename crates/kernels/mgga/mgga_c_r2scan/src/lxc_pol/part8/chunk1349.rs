//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1349/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1349<F: Float>(t23991: F, t28086: F, t28088: F, t28090: F, t25038: F, t25042: F, t28095: F, t25044: F, t28102: F, t28104: F, t19702: F, t19709: F, t19712: F, t20180: F, t23986: F, t25032: F, t25041: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t32994 = 0.17090684152272775384e-2 * t23991;
    let t32995 = 0.17544670867903938621e1 * t28086;
    let t32996 = 12.0 * t28088;
    let t32997 = 12.0 * t28090;
    let t32998 = 72.0 * t25038;
    let t32999 = 36.0 * t25042;
    let t33000 = 36.0 * t28095;
    let t33001 = 0.65061487801810439052e-1 * t25044;
    let t33002 = 24.0 * t28102;
    let t33003 = 0.73245789224026180216e-3 * t28104;
    let t33004 = -t23986 + t19702 - t32994 - t25032 - t32995 - t32996 - t32997 + t19709 + t32998 + t25041 - t19712 - t32999 - t20180 - t33000 - t33001 - t33002 + t33003;
    (t32994, t32995, t32996, t32997, t32998, t32999, t33000, t33001, t33002, t33003, t33004)
}
