//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 703/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk703<F: Float>(t3796: F, t6234: F, t3482: F, t1220: F, t1329: F, t2174: F, t3491: F, t3807: F, t3808: F, t3810: F, t412: F, t5798: F, t5977: F, t5979: F, t5983: F, t5986: F, t5989: F, t5994: F, t5999: F, t6004: F, t6009: F, t6012: F, t6218: F, t6221: F, t6227: F, t6231: F) -> (F, F, F) {
    let t6235 = t3796 * t6234;
    let t6236 = t3482 * t6235;
    let t6238 = 0.66327777777777777776e-2 * t5977 + 0.11054629629629629629e-2 * t5979 - 0.44218518518518518517e-2 * t5983 + 0.16581944444444444444e-2 * t5986 - 0.24872916666666666666e-2 * t5989 + 0.49745833333333333332e-2 * t5994 - 0.16581944444444444444e-2 * t5999 - 0.16581944444444444444e-2 * t6004 - 0.55273148148148148147e-3 * t6009 + 0.16581944444444444444e-2 * t6012 - t3807 - 0.44218518518518518517e-2 * t3808 + 0.16581944444444444444e-2 * t3810 - 0.193e0 * t3491 * t2174 - 0.193e0 * t1220 * t6218 - 0.193e0 * t6221 * t1329 + t5798 * t412 - 0.16581944444444444444e-2 * t6227 + 0.11054629629629629629e-2 * t6231 - 0.16581944444444444444e-2 * t6236;
    (t6235, t6236, t6238)
}
