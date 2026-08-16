//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1267/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1267(t54463: f64, t54491: f64, t14954: f64, t4414: f64, t14981: f64, t15004: f64, t840: f64, t54504: f64, t54531: f64, t54535: f64, t54537: f64, t54566: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t55781 = 7.0_f64 / 576.0_f64 * t54463;
    let t55796 = 7.0_f64 / 1152.0_f64 * t54491;
    let t55807 = 7.0_f64 / 72.0_f64 * t4414 * t14954;
    let t55809 = 7.0_f64 / 72.0_f64 * t4414 * t14981;
    let t55831 = 7.0_f64 / 144.0_f64 * t840 * t15004;
    let t55833 = 7.0_f64 / 72.0_f64 * t54504;
    let t55841 = 7.0_f64 / 72.0_f64 * t54531;
    let t55850 = 7.0_f64 / 36.0_f64 * t54535;
    let t55851 = 7.0_f64 / 36.0_f64 * t54537;
    let t55863 = 7.0_f64 / 36.0_f64 * t54566;
    (t55781, t55796, t55807, t55809, t55831, t55833, t55841, t55850, t55851, t55863)
}
