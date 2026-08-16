//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2319/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2319(t25365: f64, t57911: f64, t10143: f64, t1484: f64, t25374: f64, t16596: f64, t16944: f64, t16949: f64, t1877: f64, t1915: f64, t193: f64, t202: f64, t22959: f64, t23290: f64, t23295: f64, t25013: f64, t2522: f64, t25354: f64, t25358: f64, t28248: f64, t4255: f64, t4314: f64, t5544: f64, t6666: f64, t6670: f64, t67128: f64, t7541: f64, t82312: f64, t870: f64, t97999: f64, t98003: f64, t98007: f64, t98011: f64, t99042: f64) -> f64 {
    let t100562 = t57911 * t25365;
    let t100572 = t10143 * t1484 * t25374;
    let t100578 = -6.0_f64 * t1877 * t82312 * t97999 - 6.0_f64 * t2522 * t25358 * t16596 + 12.0_f64 * t4314 * t1915 * t16944 + 12.0_f64 * t4314 * t7541 * t4255 - 6.0_f64 * t2522 * t6670 * t98007 + 3.0_f64 * t2522 * t6666 * t5544 - 3.0_f64 * t2522 * t6670 * t98011 + 6.0_f64 * t2522 * t23295 * t98003 - 6.0_f64 * t2522 * t25358 * t25365 + 6.0_f64 * t2522 * t25354 * t1484 - 6.0_f64 * t4314 * t6670 * t67128 - 12.0_f64 * t25013 * t100562 + t193 * t202 * t99042 * t870 + 6.0_f64 * t4314 * t1915 * t16949 + 12.0_f64 * t22959 * t100572 - 6.0_f64 * t2522 * t23290 * t28248;
    t100578
}
