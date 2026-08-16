//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1265/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1265<F: Float>(t31832: F, t7754: F, t8689: F, t8944: F, t26164: F, t24994: F, t24996: F, t26149: F, t8690: F, t12725: F, t8675: F, t33690: F, t6535: F) -> (F, F, F, F, F, F) {
    let t123193 = t31832 * t7754;
    let t123194 = t8689 * t8944;
    let t123195 = t123194 * t26164;
    let t123198 = t8689 * t24994;
    let t123199 = t123198 * t24996;
    let t123205 = t8690 * t26149;
    let t123206 = t12725 * t8675;
    let t123211 = t33690 * t6535;
    (t123193, t123195, t123199, t123205, t123206, t123211)
}
