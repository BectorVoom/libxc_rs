//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 476/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk476(t1811: f64, t2: f64, t428: f64, t5878: f64, t68: f64, t181: f64, t4167: f64, t4169: f64, t183: f64, t155: f64, t421: f64, t4155: f64, t4163: f64, t4187: f64, t4336: f64, t5382: f64, t5385: f64, t5388: f64, t5402: f64, t5979: f64, t5981: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5983 = t1811 * t2;
    let t5984 = t5983 * t428;
    let t5985 = 0.18311447306006545054e-3_f64 * t5984;
    let t5986 = t5878 * t68;
    let t5988 = 0.19751673498613801407e-1_f64 * t5986 * t181;
    let t5989 = 0.5848223622634646207e0_f64 * t4167;
    let t5990 = 0.17315859105681463759e2_f64 * t4169;
    let t5991 = t5986 * t183;
    let t5992 = t155 * t5991;
    let t5993 = t1811 * t421;
    let t5994 = t155 * t5993;
    let t5995 = t5979 - t5382 + t5981 - t5385 + t5388 - t5985 + t5988 - t4155 - t4163 - t5989 - t5990 - t5402 + t5992 + t5994 + t4187 + t4336;
    (t5985, t5988, t5989, t5990, t5992, t5994, t5995)
}
