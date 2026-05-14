//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1352/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1352<F: Float>(t35089: F, t5074: F, t1799: F, t34093: F, t6981: F, t116201: F, t116416: F, t116482: F, t116513: F, t116790: F, t1775: F, t22592: F, t32942: F, t32990: F, t34023: F, t34027: F, t34039: F, t34122: F, t34133: F, t35159: F, t35163: F, t9664: F, t9665: F, t9936: F) -> (F, F, F) {
    let t121102 = t5074 * t35089;
    let t121105 = t1799 * t34093 * t6981;
    let t121107 = -0.18518518518518518519e-1 * t116482 * t34027 + 0.69444444444444444447e-2 * t116513 * t34023 - 0.37037037037037037038e-1 * t116482 * t34039 - 0.34722222222222222223e-2 * t32942 * t35159 - 0.34722222222222222223e-2 * t32990 * t35159 - 0.34722222222222222223e-2 * t9664 * t1775 * t9665 * t22592 - 0.46296296296296296297e-2 * t32942 * t35163 - 0.46296296296296296297e-2 * t32990 * t35163 - 0.69444444444444444446e-2 * t116790 * t9936 - 0.69444444444444444446e-2 * t116201 * t9936 + 0.13888888888888888889e-1 * t34122 * t34133 + 0.18518518518518518519e-1 * t116416 * t9936 - 0.22109259259259259259e-2 * t121102 - 0.58958024691358024689e-2 * t121105;
    (t121102, t121105, t121107)
}
