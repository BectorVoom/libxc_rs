//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 862/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk862<F: Float>(t10471: F, t140: F, t416: F, t1442: F, t1452: F, t3496: F, t3739: F, t3744: F, t3748: F, t3766: F, t1286: F, t3786: F, t1450: F, t3785: F, t1411: F, t14201: F, t14206: F, t14211: F, t14216: F, t14218: F, t14220: F) -> (F, F, F, F, F, F, F, F) {
    let t14223 = t140 * t10471 * t416;
    let t14224 = t14223 * t1442;
    let t14226 = t14223 * t1452;
    let t14228 = t3739 * t3496;
    let t14230 = t3739 * t3744;
    let t14232 = t3748 * t3766;
    let t14234 = t3786 * t1286;
    let t14235 = t1450 * t14234;
    let t14236 = t3785 * t14235;
    let t14237 = t1411 * t14236;
    let t14239 = 0.2653111111111111111e-1 * t14201 - 0.49745833333333333332e-2 * t14206 + 0.73697530864197530862e-3 * t14211 + 0.44218518518518518518e-2 * t14216 + 0.33163888888888888887e-2 * t14218 - 0.99491666666666666664e-2 * t14220 + 0.16581944444444444444e-2 * t14224 - 0.11054629629629629629e-2 * t14226 - 0.17687407407407407407e-1 * t14228 - 0.66327777777777777776e-2 * t14230 - 0.17687407407407407407e-1 * t14232 + 0.99491666666666666664e-2 * t14237;
    (t14224, t14226, t14228, t14230, t14232, t14234, t14237, t14239)
}
