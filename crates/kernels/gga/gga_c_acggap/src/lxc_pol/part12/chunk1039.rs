//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1039/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1039<F: Float>(t35180: F, t35186: F, t35194: F, t30991: F, t30993: F, t30998: F, t31003: F, t32664: F, t32668: F, t32670: F, t32671: F, t32672: F, t35172: F, t35176: F, t35184: F, t35190: F, t35198: F, t35200: F) -> (F,) {
    let t37426 = 0.21437009059034868486e-3 * t35180;
    let t37428 = 0.12862205435420921092e-1 * t35186;
    let t37430 = 0.37737710747524982482e-2 * t35194;
    let t37433 = -t32664 - 0.12862205435420921092e-2 * t30991 - 0.38110238327173099532e-2 * t30993 - 0.41930789719472202758e-3 * t30998 + t32668 - 0.17149607247227894789e-2 * t31003 + t32670 - t32671 + t32672 - 0.12579236915841660828e-2 * t35172 - 0.83861579438944405517e-3 * t35176 + t37426 - 0.41930789719472202758e-3 * t35184 - t37428 + 0.94344276868812456208e-2 * t35190 - t37430 + 0.75475421495049964966e-2 * t35198 + 0.68598428988911579156e-2 * t35200;
    (t37433,)
}
