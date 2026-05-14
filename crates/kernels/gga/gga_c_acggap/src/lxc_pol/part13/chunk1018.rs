//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1018/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1018<F: Float>(t17912: F, t2288: F, t31443: F, t3169: F, t31598: F, t31602: F, t35766: F, t35768: F, t35772: F, t35775: F, t35778: F, t35782: F, t35785: F, t35789: F, t35790: F, t35792: F, t35795: F, t35798: F, t35800: F, t35801: F, t35804: F) -> (F,) {
    let t35808 = t31443 * t17912 * t2288 * t3169;
    let t35810 = 0.68598428988911579156e-2 * t35766 - 0.68598428988911579156e-2 * t35768 - t31598 - t31602 + 0.64311027177104605458e-3 * t35772 + t35775 + 0.21437009059034868486e-2 * t35778 + 0.12862205435420921092e-2 * t35782 + t35785 + t35789 + 0.85748036236139473944e-3 * t35790 - 0.85748036236139473945e-2 * t35792 - t35795 + t35798 + t35800 - 0.20579528696673473746e-1 * t35801 + 0.47172138434406228102e-2 * t35804 - 0.18868855373762491241e-2 * t35808;
    (t35810,)
}
