//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1837/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1837(t14363: f64, t324: f64, t2924: f64, t4475: f64, t10632: f64, t1580: f64, t2906: f64, t10756: f64, t10820: f64, t13729: f64, t14257: f64, t14329: f64, t14332: f64, t14337: f64, t14344: f64, t1581: f64, t2856: f64, t2900: f64, t2925: f64, t2930: f64, t2933: f64, t4434: f64, t4449: f64, t4472: f64, t924: f64, t943: f64, t952: f64) -> (f64, f64, f64, f64, f64) {
    let t14364 = t14363 * t324;
    let t14366 = t4475 * t2924;
    let t14369 = t1580 * t10632;
    let t14370 = t14369 * t2906;
    let t14373 = 2.0_f64 * t2856 * t4434 + 1.0_f64 * t924 * t14329 + 0.11696447245269292414e1_f64 * t14332 * t952 + 0.5848223622634646207e0_f64 * t4449 * t2925 + 0.17315859105681463759e2_f64 * t14337 * t2933 + 0.5848223622634646207e0_f64 * t10820 * t1581 + 0.11696447245269292414e1_f64 * t2900 * t4472 + 0.5848223622634646207e0_f64 * t943 * t14344 + t13729 + t14257 - 0.19751673498613801407e-1_f64 * t14364 + 0.17315859105681463759e2_f64 * t2930 * t14366 + 0.10254018858216406658e4_f64 * t10756 * t14370;
    (t14364, t14366, t14369, t14370, t14373)
}
