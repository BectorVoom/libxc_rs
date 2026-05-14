//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1343/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1343<F: Float>(t25387: F, t415: F, t468: F, t33557: F, t5968: F, t25329: F, t9469: F, t1308: F, t20886: F, t2158: F, t12841: F, t34701: F, t110068: F, t113735: F, t113941: F, t114038: F, t1220: F, t20: F, t20111: F, t20116: F, t25342: F, t25413: F, t26041: F, t26422: F, t2714: F, t2718: F, t32087: F, t33415: F, t33424: F, t34794: F, t3491: F, t394: F, t80804: F, t9429: F) -> (F, F, F, F, F) {
    let t119608 = t415 * t468 * t25387;
    let t119614 = t415 * t33557 * t5968;
    let t119618 = t415 * t9469 * t25329;
    let t119624 = t20886 * t2158 * t1308;
    let t119627 = t12841 * t34701;
    let t119629 = -0.10802469135802469136e-1 * t32087 * t20111 * t114038 * t25342 - 0.18518518518518518519e-1 * t32087 * t20116 * t33415 * t25413 + 0.69444444444444444447e-2 * t113941 * t33424 - 0.10416666666666666667e-1 * t1220 * t26422 * t394 * t20 * t2718 - 0.10416666666666666667e-1 * t3491 * t34794 * t2718 + 0.14739506172839506173e-2 * t119608 - 0.13888888888888888889e-1 * t32087 * t113735 * t26041 - 0.49745833333333333332e-2 * t119614 + 0.55273148148148148147e-3 * t110068 + 0.66327777777777777776e-2 * t119618 - 0.10416666666666666667e-1 * t80804 * t2714 * t2718 + 0.80416666666666666669e-2 * t119624 * t9429 + 0.18424382716049382715e-2 * t119627;
    (t119608, t119614, t119618, t119627, t119629)
}
