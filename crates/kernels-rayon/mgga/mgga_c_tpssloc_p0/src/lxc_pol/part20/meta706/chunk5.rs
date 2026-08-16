//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2695/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2695(t1372: f64, t5286: f64, t1824: f64, t3879: f64, t12240: f64, t1351: f64, t16205: f64, t562: f64, t12168: f64, t1352: f64, t16036: f64, t16040: f64, t16041: f64, t16047: f64, t16048: f64, t16055: f64, t26409: f64, t3773: f64, t3793: f64, t3851: f64, t3856: f64, t5333: f64, t5334: f64, t5335: f64, t5336: f64, t5343: f64, t5344: f64, t5345: f64) -> (f64, f64, f64, f64, f64) {
    let t54840 = t1372 * t5286;
    let t54854 = t3879 * t1824;
    let t54858 = t12240 * t1351;
    let t54883 = t562 * t16205;
    let t54900 = -t12168 * t5335 * t5344 + 6.0_f64 * t12240 * t16036 * t5334 - 3.0_f64 * t1352 * t5344 * t54883 - 18.0_f64 * t16036 * t16047 * t16048 + 18.0_f64 * t16036 * t3793 * t5334 + 18.0_f64 * t16040 * t3793 * t5334 - 3.0_f64 * t16040 * t3856 * t5344 - 3.0_f64 * t26409 * t3851 * t5344 + 6.0_f64 * t3773 * t5333 * t5336 - 3.0_f64 * t3773 * t5343 * t5345 + 12.0_f64 * t16041 * t16055;
    (t54840, t54854, t54858, t54883, t54900)
}
