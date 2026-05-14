//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 478/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk478<F: Float>(t155: F, t5428: F, t1042: F, t1372: F, t1138: F, t1435: F, t1392: F, t446: F, t589: F, t998: F, t1143: F, t1425: F, t1535: F, t4232: F, t4252: F, t4255: F, t4259: F, t4290: F, t4351: F, t4580: F, t5412: F, t5417: F, t5418: F, t5420: F, t5421: F, t5422: F, t5426: F, t5427: F) -> (F, F, F, F, F, F) {
    let t5429 = t155 * t5428;
    let t5432 = t1372 * t1042;
    let t5433 = 0.17315859105681463759e2 * t5432;
    let t5434 = t1435 * t1138;
    let t5435 = 0.24415263074675393405e-3 * t5434;
    let t5436 = t1392 * t446;
    let t5439 = t589 * t998;
    let t5442 = 0.186546e0 * t5412 * t589 + t5417 + t5418 + t5420 - t5421 - 0.186546e0 * t1425 * t5422 + t4232 + t4252 - t4255 - t4259 + t5426 - t4351 + t5427 + t5429 + t4290 + 0.93273e-1 * t4580 * t1535 - t5433 + t5435 + 0.373092e0 * t1143 * t5436 + 0.186546e0 * t1143 * t5439;
    (t5429, t5433, t5435, t5436, t5439, t5442)
}
