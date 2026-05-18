//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 998/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk998<F: Float>(t12019: F, t374: F, t11657: F, t11660: F, t11687: F, t11700: F, t11753: F, t11758: F, t11762: F, t11766: F, t11772: F, t11774: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12020 = t12019 * t374;
    let t12120 = F::new(0.23115257973478049502e0) * t11657;
    let t12121 = F::new(0.46574606203128791246e-1) * t11660;
    let t12132 = F::new(0.23115257973478049502e0) * t11687;
    let t12138 = F::new(0.14282990759302185292e-1) * t11700;
    let t12158 = F::new(0.19514881078765566037e-1) * t11753;
    let t12162 = F::new(0.54878743191129263322e-2) * t11758;
    let t12163 = F::new(0.46574606203128791246e-1) * t11762;
    let t12164 = F::new(0.13972381860938637374e0) * t11766;
    let t12166 = F::new(0.46574606203128791246e-1) * t11772;
    let t12167 = F::new(0.10975748638225852664e-1) * t11774;
    (t12020, t12120, t12121, t12132, t12138, t12158, t12162, t12163, t12164, t12166, t12167)
}
