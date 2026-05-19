//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1088/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1088<F: Float>(t47825: F, t7717: F, t1707: F, t2124: F, t42024: F, t42027: F, t42035: F, t42042: F, t42044: F, t43990: F, t47795: F, t47797: F, t47800: F, t47802: F, t47804: F, t47809: F, t47814: F, t47816: F, t47818: F, t47821: F, t903: F) -> F {
    let t47826 = t7717 * t47825;
    let t47828 = F::cast_from(0.35922725105591425692e0_f64) * t903 * t2124 * t1707 - t42024 - t42027 - F::cast_from(0.5987120850931904282e-1_f64) * t47795 - t42035 + F::cast_from(0.8980681276397856423e-1_f64) * t47797 + F::cast_from(0.60975299583150056628e-3_f64) * t42042 - F::cast_from(0.20455996240684006296e-1_f64) * t47800 - F::cast_from(0.81823984962736025184e-1_f64) * t47802 - F::cast_from(0.20455996240684006296e-1_f64) * t47804 - F::cast_from(0.17025839957319135759e-4_f64) * t47809 + F::cast_from(0.85129199786595678796e-5_f64) * t47814 + F::cast_from(0.1064114997332445985e-4_f64) * t47816 + F::new(2.0) * t47818 + F::cast_from(0.59590439850616975158e-4_f64) * t42044 + F::cast_from(0.19863479950205658386e-4_f64) * t47821 + F::cast_from(0.1064114997332445985e-4_f64) * t47826 + t43990;
    t47828
}
