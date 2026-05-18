//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1117/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1117<F: Float>(t41523: F, t41531: F, t41534: F, t41536: F, t27055: F, t27094: F, t321: F, t352: F, t36278: F, t36284: F, t36286: F, t36294: F, t44183: F, t44187: F, t5259: F, t839: F, t876: F, t8940: F, t9540: F) -> F {
    let t44337 = F::new(0.47896966807455234256e0) * t41523;
    let t44339 = F::new(0.95793933614910468512e0) * t41531;
    let t44340 = F::new(0.19158786722982093702e1) * t41534;
    let t44341 = F::new(0.47896966807455234256e0) * t41536;
    let t44342 = F::new(0.1454648621559751559e0) * t36278 + F::new(0.23948483403727617128e0) * t8940 * t44183 * t352 + F::new(0.23948483403727617128e0) * t5259 * t44187 * t321 - F::new(0.35922725105591425692e0) * t27055 * t9540 * t876 - F::new(0.11974241701863808564e1) * t27094 * t9540 * t839 + F::new(0.11708147441822390596e1) * t36284 - F::new(0.17562221162733585894e1) * t36286 + t44337 - F::new(0.15965655602485078085e0) * t36294 + t44339 - t44340 - t44341;
    t44342
}
