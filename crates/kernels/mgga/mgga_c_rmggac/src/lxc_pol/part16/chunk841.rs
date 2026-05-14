//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 841/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk841<F: Float>(t2412: F, t8582: F, t2191: F, t9790: F, t9938: F, t10040: F, t2004: F, t9826: F, t2007: F, t1987: F, t7501: F, t9799: F, t2139: F, t27: F, t6376: F, t649: F) -> (F, F, F, F, F, F, F, F, F) {
    let t45844 = t2412 * t8582;
    let t45846 = t2191 * t9790;
    let t45864 = t2191 * t9938;
    let t45866 = t2191 * t10040;
    let t45869 = t9826 * t2004;
    let t45872 = t9826 * t2007;
    let t45874 = t9826 * t1987;
    let t45880 = t7501 * t9799;
    let t45884 = t2139 * t27 * t649 * t6376;
    (t45844, t45846, t45864, t45866, t45869, t45872, t45874, t45880, t45884)
}
