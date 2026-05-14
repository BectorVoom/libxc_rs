//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 950/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk950<F: Float>(t2665: F, t3746: F, t6318: F, t28755: F, t684: F, t7036: F, t24976: F, t6317: F, t28516: F, t24974: F, t24987: F, t28722: F, t28727: F, t28732: F, t28739: F, t28744: F, t28749: F, t28753: F) -> (F, F, F, F, F, F, F, F) {
    let t28757 = t2665 * t6318 * t3746;
    let t28758 = t28755 * t28757;
    let t28760 = t7036 * t684;
    let t28761 = t24976 * t28760;
    let t28762 = t6317 * t28761;
    let t28764 = t24976 * t28516;
    let t28765 = t6317 * t28764;
    let t28767 = -t28722 / 3.0 - t24974 / 36.0 - t28727 / 36.0 - t28732 / 36.0 - 2.0 / 9.0 * t24987 - t28739 / 8.0 - t28744 / 6.0 + t28749 / 18.0 + t28753 / 18.0 - t28758 / 9.0 - t28762 / 9.0 - t28765 / 9.0;
    (t28757, t28758, t28760, t28761, t28762, t28764, t28765, t28767)
}
