//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1974/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1974<F: Float>(t87028: F, t87066: F, t87068: F, t87080: F, t87100: F, t81571: F, t81575: F, t81592: F, t87055: F, t87059: F, t87076: F, t87078: F, t87084: F, t87092: F, t87097: F, t87104: F, t87109: F, t87114: F) -> (F, F) {
    let t92486 = F::cast_from(0.3289868133696452873e-1_f64) * t87028;
    let t92491 = F::cast_from(0.76763589786250567036e-1_f64) * t87066;
    let t92492 = F::cast_from(0.52089578783527170489e-1_f64) * t87068;
    let t92497 = F::cast_from(0.12793931631041761173e0_f64) * t87080;
    let t92502 = F::cast_from(0.16449340668482264365e-1_f64) * t87100;
    let t92506 = F::cast_from(0.6579736267392905746e-1_f64) * t87055 - F::cast_from(0.19739208802178717238e0_f64) * t87059 + t92491 - t92492 - F::cast_from(0.82246703342411321825e-2_f64) * t81571 + F::cast_from(0.6579736267392905746e-1_f64) * t81575 - F::cast_from(0.9869604401089358619e-1_f64) * t87076 - F::cast_from(0.46058153871750340222e0_f64) * t87078 + t92497 + F::cast_from(0.6579736267392905746e-1_f64) * t87084 - F::cast_from(0.15352717957250113407e0_f64) * t81592 + F::cast_from(0.16449340668482264365e-1_f64) * t87092 - F::cast_from(0.16449340668482264365e-1_f64) * t87097 + t92502 + F::cast_from(0.9869604401089358619e-1_f64) * t87104 - F::cast_from(0.16449340668482264365e-1_f64) * t87109 + F::cast_from(0.19739208802178717238e0_f64) * t87114;
    (t92486, t92506)
}
