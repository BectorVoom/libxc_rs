//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1247/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1247<F: Float>(t12844: F, t7417: F, t4439: F, t68: F, t7402: F, t610: F, t4425: F, t7425: F, t1599: F, t4455: F, t7492: F, t1610: F, t6176: F, t1603: F, t18142: F, t18148: F, t18152: F, t18164: F, t18170: F, t18174: F, t18178: F, t18205: F, t18213: F, t6141: F, t6165: F) -> (F, F) {
    let t23154 = t12844 * t7417;
    let t23155 = t4439 * t23154;
    let t23157 = t7402 * t68;
    let t23158 = t610 * t23157;
    let t23163 = t4425 * t7425;
    let t23164 = t1599 * t23163;
    let t23167 = t4455 * t7492;
    let t23168 = t23167 * t1610;
    let t23169 = t6176 * t23168;
    let t23172 = t18142 / 432.0 - t18148 + t18152 - t23155 / 864.0 + 11.0 / 648.0 * t23158 * t1603 + t6141 * t6165 / 54.0 + t23164 / 1728.0 - t18164 / 1296.0 - t18170 - t18174 + t18178 - t18205 + t18213 + t1599 * t23169 / 96.0;
    (t23158, t23172)
}
