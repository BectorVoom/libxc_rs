//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1037/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1037<F: Float>(t2752: F, t28: F, t13487: F, t1081: F, t776: F, t2553: F, t2749: F, t868: F, t2745: F, t12461: F, t3698: F, t2039: F, t3652: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23788 = t2752 * t28;
    let t23789 = t23788 * t13487;
    let t23792 = t1081 * t776;
    let t23796 = t28 * t2553;
    let t23807 = t28 * t2749;
    let t23810 = t1081 * t868;
    let t23813 = t28 * t2745;
    let t23857 = t12461 * t3698;
    let t23909 = t3652 * t2039;
    (t23788, t23789, t23792, t23796, t23807, t23810, t23813, t23857, t23909)
}
