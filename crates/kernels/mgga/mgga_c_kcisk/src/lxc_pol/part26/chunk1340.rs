//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1340/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1340<F: Float>(t1339: F, t33604: F, t5627: F, t1286: F, t1411: F, t32045: F, t80875: F, t114618: F, t6234: F, t114597: F, t114606: F, t114625: F, t118827: F, t118938: F, t119298: F, t32026: F, t32087: F, t32189: F, t33377: F, t33400: F, t33439: F, t33460: F, t34697: F, t34744: F, t9426: F, t9446: F) -> (F, F, F, F) {
    let t119534 = t1339 * t33604 * t5627;
    let t119540 = t1411 * t32045 * t80875 * t1286;
    let t119548 = t1339 * t114618 * t6234;
    let t119557 = -0.69444444444444444447e-2 * t32087 * t119298 - 0.66327777777777777776e-2 * t119534 + 0.21444444444444444445e-1 * t32189 * t34744 - 0.16581944444444444444e-2 * t119540 - t114597 + 0.40208333333333333335e-2 * t32026 * t34697 + 0.22109259259259259259e-2 * t114606 - 0.40208333333333333335e-2 * t9426 * t118827 - 0.33163888888888888888e-2 * t119548 - 0.80416666666666666669e-2 * t33460 * t33439 - 0.20833333333333333334e-1 * t9446 * t118938 - 0.8041666666666666667e-2 * t33377 * t33400 + 0.11054629629629629629e-2 * t114625;
    (t119534, t119540, t119548, t119557)
}
