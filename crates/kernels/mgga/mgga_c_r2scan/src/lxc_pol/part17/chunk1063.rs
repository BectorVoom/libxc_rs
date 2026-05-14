//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1063/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1063<F: Float>(t12052: F, t12422: F, t2867: F, t3275: F, t41202: F, t44061: F, t44064: F, t44068: F, t44072: F, t44074: F, t44077: F, t44080: F, t44083: F, t44086: F, t44089: F, t44091: F, t44093: F, t44096: F, t44098: F) -> (F, F, F) {
    let t44100 = t12422 * t12052 / 4.0;
    let t44103 = t3275 * t41202 * t2867 / 2.0;
    let t44104 = -t44061 + t44064 - t44068 - t44072 - t44074 + t44077 + t44080 + t44083 - t44086 - t44089 + t44091 - t44093 + t44096 - t44098 + t44100 - t44103;
    (t44100, t44103, t44104)
}
