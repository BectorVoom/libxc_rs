//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 952/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk952<F: Float>(t2004: F, t9826: F, t2007: F, t1987: F, t7501: F, t9799: F, t2139: F, t27: F, t6376: F, t649: F, t2001: F, t2281: F, t305: F, t551: F) -> (F, F, F, F, F, F) {
    let t45869 = t9826 * t2004;
    let t45872 = t9826 * t2007;
    let t45874 = t9826 * t1987;
    let t45880 = t7501 * t9799;
    let t45884 = t2139 * t27 * t649 * t6376;
    let t45889 = t2001 * t305 * t2281 * t551;
    (t45869, t45872, t45874, t45880, t45884, t45889)
}
