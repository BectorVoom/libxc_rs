//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 731/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk731<F: Float>(t2019: F, t34790: F, t4789: F, t640: F, t649: F, t49: F, t288: F, t290: F, t2010: F, t2139: F, t27: F, t3118: F, t333: F) -> (F, F, F, F, F) {
    let t34793 = t2019 * t649 * t4789 * t640 * t34790;
    let t34794 = F::cast_from(0.91462949374725084942e-3_f64) * t34793;
    let t34795 = t4789 * t49;
    let t34796 = t34795 * t288;
    let t34797 = t290 * t34790;
    let t34799 = t2010 * t34796 * t34797;
    let t34803 = t2139 * t27 * t3118 * t333;
    (t34794, t34795, t34797, t34799, t34803)
}
