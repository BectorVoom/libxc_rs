//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 627/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk627<F: Float>(t1131: F, t4984: F, t1096: F, t1092: F, t1713: F, t2825: F, t1020: F, t251: F, t66: F, t1018: F, t86: F) -> (F, F, F, F, F, F, F) {
    let t4985 = t1131 * t4984;
    let t4986 = t1096 * t4985;
    let t4987 = t1092 * t4986;
    let t4989 = t2825 * t1713;
    let t4990 = t1020 * t4989;
    let t4992 = t66 * t251;
    let t4994 = t86 * t4992 * t1018;
    (t4985, t4986, t4987, t4989, t4990, t4992, t4994)
}
