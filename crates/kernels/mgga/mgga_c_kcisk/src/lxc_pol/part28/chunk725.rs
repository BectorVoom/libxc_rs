//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 725/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk725<F: Float>(t673: F, t8662: F, t140: F, t1470: F, t2517: F, t2521: F, t2543: F, t479: F, t5231: F, t5242: F, t709: F, t725: F, t7368: F, t7387: F, t8915: F, t8919: F, t8923: F, t8927: F, t8931: F, t8975: F, t8995: F, t8999: F, t9003: F, t9007: F) -> (F, F) {
    let t9010 = t673 * t8662;
    let t9014 = 0.619125e-2 * t8975 * t709 + 0.1857375e-1 * t2543 * t2517 - 0.123825e-1 * t2543 * t2521 + 0.46434375e-2 * t725 * t8915 - 0.1857375e-1 * t5231 * t8919 + 0.9286875e-2 * t725 * t8923 + 0.123825e-1 * t725 * t8927 - 0.619125e-2 * t725 * t8931 + t5242 - 0.35374814814814814814e-1 * t7368 - 0.53062222222222222222e-1 * t7387 - 0.44218518518518518518e-1 * t1470 * t8995 - 0.53062222222222222222e-1 * t1470 * t8999 + 0.53062222222222222222e-1 * t1470 * t9003 - 0.26531111111111111111e-1 * t1470 * t9007 - 0.39796666666666666666e-1 * t140 * t479 * t9010;
    (t9010, t9014)
}
