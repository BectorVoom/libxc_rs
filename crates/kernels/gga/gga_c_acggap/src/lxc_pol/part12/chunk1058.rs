//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1058/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1058<F: Float>(t35909: F, t35911: F, t35913: F, t35915: F, t35917: F, t35919: F, t31629: F, t31632: F, t31634: F, t31638: F, t31640: F, t31644: F, t31646: F, t31658: F, t31660: F, t32891: F, t35907: F) -> (F,) {
    let t37777 = 0.916875e-1 * t35909;
    let t37778 = 0.916875e-1 * t35911;
    let t37779 = 0.61125e-1 * t35913;
    let t37780 = 0.61125e-1 * t35915;
    let t37781 = 0.34299214494455789578e-2 * t35917;
    let t37782 = 0.34299214494455789578e-2 * t35919;
    let t37785 = 0.25724410870841842184e-1 * t31629 - 0.16006300097412701803e-1 * t31632 - 0.12862205435420921092e-1 * t31634 + 0.94344276868812456206e-2 * t31638 - 0.17149607247227894789e-1 * t31640 - 0.45351183609335988442e-1 * t31644 - 0.64025200389650807212e-1 * t31646 - 0.21437009059034868486e-2 * t35907 + t37777 + t37778 + t37779 + t37780 - t37781 + t37782 - 0.16772315887788881103e-1 * t31658 + 0.18868855373762491241e-2 * t31660 + t32891;
    (t37785,)
}
