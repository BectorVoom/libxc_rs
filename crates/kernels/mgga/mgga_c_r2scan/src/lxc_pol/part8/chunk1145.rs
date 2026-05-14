//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1145/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1145<F: Float>(t1399: F, t1933: F, t1939: F, t197: F, t200: F, t206: F, t21272: F, t21276: F, t21279: F, t21354: F, t21357: F, t21361: F, t21365: F, t21370: F, t21416: F, t390: F, t4741: F, t5319: F, t5629: F, t5633: F, t5693: F, t5696: F, t63: F) -> (F,) {
    let t21580 = t21272 + t21276 + 0.28343096359072795448e3 * t1399 * t5629 + 0.44060335298551228072e1 * t1399 * t5633 - t21279 + 0.25685571960238451669e8 * t63 / t5693 / t200 * t206 / t5696 / t197 * t21416 - t21354 + t21357 + t21361 + t21365 + t21370 + 0.42618074074074074072e0 * t4741 * t1933 - 0.68538299353301910335e1 * t4741 * t1939 - 0.64212977516902094772e0 * t390 * t5319;
    (t21580,)
}
