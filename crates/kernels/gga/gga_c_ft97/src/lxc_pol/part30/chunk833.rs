//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 833/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk833<F: Float>(t24357: F, t33380: F, t173: F, t33373: F, t27521: F, t7470: F, t1418: F, t33372: F, t136595: F, t27519: F, t3771: F, t24260: F, t3766: F, t9: F, t420: F, t6044: F) -> (F, F, F, F, F, F) {
    let t141071 = t33380 * t24357;
    let t141073 = t173 * t33373;
    let t141075 = t27521 * t7470 * t141073;
    let t141082 = t33372 * t1418 * t141073;
    let t141089 = t3771 * t27519 * t136595;
    let t141096 = t3766 * t24260 * t9;
    let t141097 = t6044 * t420;
    (t141071, t141075, t141082, t141089, t141096, t141097)
}
