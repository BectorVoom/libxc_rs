//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1422/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1422<F: Float>(t26259: F, t26295: F, t26297: F, t26301: F, t30765: F, t30772: F, t30777: F, t30779: F, t30787: F, t30789: F, t30793: F, t30796: F, t30801: F, t34463: F, t10359: F, t2201: F, t785: F, t788: F) -> (F, F) {
    let t34474 = -t26259 - 0.14636160809074174528e-1 * t34463 + 0.34930954652346593433e-1 * t30765 + 0.16463622957338778996e-1 * t30772 + 0.41607464352260489103e1 * t30777 - 0.20803732176130244552e1 * t30779 + t26295 + t26297 + 0.34930954652346593433e-1 * t30787 + 0.1047928639570397803e0 * t30789 + 0.4191714558281591212e0 * t30793 + 0.2037639021386884617e0 * t30796 + t26301 - 0.2037639021386884617e0 * t30801;
    let t34479 = t2201 * t785 * t788 * t10359;
    (t34474, t34479)
}
