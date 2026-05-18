//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1151/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1151<F: Float>(t6284: F, t7909: F, t5709: F, t27438: F, t6281: F, t5701: F, t28356: F, t8164: F, t1394: F, t5653: F, t7923: F, t2243: F, t7193: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t29357 = t7909 * t6284;
    let t29358 = t5709 * t29357;
    let t29361 = t27438 * t6281;
    let t29362 = t5701 * t29361;
    let t29365 = t28356 * t8164;
    let t29366 = t1394 * t29365;
    let t29368 = t5653 * t6281;
    let t29369 = t7923 * t29368;
    let t29370 = t1394 * t29369;
    let t29372 = t7193 * t2243;
    (t29357, t29358, t29361, t29362, t29365, t29366, t29368, t29369, t29370, t29372)
}
