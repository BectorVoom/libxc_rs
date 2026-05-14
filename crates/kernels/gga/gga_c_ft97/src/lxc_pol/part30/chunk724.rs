//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 724/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk724<F: Float>(t34012: F, t875: F, t6353: F, t6386: F, t10688: F, t7672: F, t2749: F, t7679: F, t33966: F, t6223: F, t193: F, t25465: F, t6222: F, t33818: F, t33845: F, t33815: F, t33825: F, t33833: F, t33838: F, t33842: F, t33850: F, t33854: F, t33857: F, t33862: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t34013 = t34012 * t875;
    let t34015 = t6353 * t6386;
    let t34017 = t10688 * t7672;
    let t34019 = t2749 * t7679;
    let t34021 = t33966 * t6223;
    let t34022 = t193 * t34021;
    let t34024 = t6222 * t25465;
    let t34025 = t193 * t34024;
    let t34031 = 2.0 / 3.0 * t33818;
    let t34036 = t33845 / 3.0;
    let t34041 = 3.0 / 2.0 * t33815 + t34031 + 2.0 / 3.0 * t33825 + 4.0 * t33833 - 2.0 * t33838 - t33842 / 2.0 - t34036 - t33850 / 3.0 - 3.0 * t33854 + 2.0 * t33857 + t33862 / 4.0;
    (t34013, t34015, t34017, t34019, t34021, t34022, t34024, t34025, t34031, t34036, t34041)
}
