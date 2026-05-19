//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 476/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk476<F: Float>(t1811: F, t2: F, t428: F, t5878: F, t68: F, t181: F, t4167: F, t4169: F, t183: F, t155: F, t421: F, t4155: F, t4163: F, t4187: F, t4336: F, t5382: F, t5385: F, t5388: F, t5402: F, t5979: F, t5981: F) -> (F, F, F, F, F, F, F) {
    let t5983 = t1811 * t2;
    let t5984 = t5983 * t428;
    let t5985 = F::cast_from(0.18311447306006545054e-3_f64) * t5984;
    let t5986 = t5878 * t68;
    let t5988 = F::cast_from(0.19751673498613801407e-1_f64) * t5986 * t181;
    let t5989 = F::cast_from(0.5848223622634646207e0_f64) * t4167;
    let t5990 = F::cast_from(0.17315859105681463759e2_f64) * t4169;
    let t5991 = t5986 * t183;
    let t5992 = t155 * t5991;
    let t5993 = t1811 * t421;
    let t5994 = t155 * t5993;
    let t5995 = t5979 - t5382 + t5981 - t5385 + t5388 - t5985 + t5988 - t4155 - t4163 - t5989 - t5990 - t5402 + t5992 + t5994 + t4187 + t4336;
    (t5985, t5988, t5989, t5990, t5992, t5994, t5995)
}
