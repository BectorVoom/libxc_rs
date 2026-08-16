//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1117/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1117(t41523: f64, t41531: f64, t41534: f64, t41536: f64, t27055: f64, t27094: f64, t321: f64, t352: f64, t36278: f64, t36284: f64, t36286: f64, t36294: f64, t44183: f64, t44187: f64, t5259: f64, t839: f64, t876: f64, t8940: f64, t9540: f64) -> f64 {
    let t44337 = 0.47896966807455234256e0_f64 * t41523;
    let t44339 = 0.95793933614910468512e0_f64 * t41531;
    let t44340 = 0.19158786722982093702e1_f64 * t41534;
    let t44341 = 0.47896966807455234256e0_f64 * t41536;
    let t44342 = 0.1454648621559751559e0_f64 * t36278 + 0.23948483403727617128e0_f64 * t8940 * t44183 * t352 + 0.23948483403727617128e0_f64 * t5259 * t44187 * t321 - 0.35922725105591425692e0_f64 * t27055 * t9540 * t876 - 0.11974241701863808564e1_f64 * t27094 * t9540 * t839 + 0.11708147441822390596e1_f64 * t36284 - 0.17562221162733585894e1_f64 * t36286 + t44337 - 0.15965655602485078085e0_f64 * t36294 + t44339 - t44340 - t44341;
    t44342
}
