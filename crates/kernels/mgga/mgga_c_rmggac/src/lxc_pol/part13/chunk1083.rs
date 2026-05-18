//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1083/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1083<F: Float>(t41297: F, t41308: F, t41314: F, t41319: F, t41323: F, t41294: F, t41299: F, t41302: F, t41305: F, t41311: F, t41317: F, t41321: F, t41325: F, t41327: F, t41330: F, t41332: F) -> F {
    let t43588 = F::new(0.24244143692662525982e0) * t41297;
    let t43592 = F::new(0.14546486215597515589e0) * t41308;
    let t43594 = F::new(0.14546486215597515589e0) * t41314;
    let t43596 = F::new(0.4838420607177634088e-2) * t41319;
    let t43598 = F::new(0.67737888500486877232e-2) * t41323;
    let t43603 = F::new(0.16934472125121719308e-2) * t41294 - t43588 - F::new(0.90317184667315836309e-2) * t41299 - F::new(0.72732431077987577945e-1) * t41302 + F::new(0.13637330827122670865e0) * t41305 + t43592 - F::new(0.5454932330849068346e-1) * t41311 + t43594 - F::new(0.2727466165424534173e0) * t41317 - t43596 + F::new(0.10160683275073031585e-1) * t41321 + t43598 - F::new(0.63504270469206447405e-2) * t41325 + F::new(0.67737888500486877232e-2) * t41327 - F::new(0.15241024912609547377e-1) * t41330 + F::new(0.5987120850931904282e-1) * t41332;
    t43603
}
