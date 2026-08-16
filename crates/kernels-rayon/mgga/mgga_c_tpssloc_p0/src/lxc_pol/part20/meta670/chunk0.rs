//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2517/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2517(t136: f64, t3297: f64, t50964: f64, t2403: f64, t4772: f64, t14792: f64, t699: f64, t1113: f64, t50929: f64, t50826: f64, t50919: f64, t43727: f64, t43729: f64, t43748: f64, t43750: f64, t50828: f64, t50832: f64, t50834: f64, t50897: f64, t50900: f64, t50903: f64, t50905: f64, t50907: f64, t50912: f64, t50917: f64, t50921: f64, t50926: f64, t50931: f64, t50934: f64) -> (f64, f64, f64, f64, f64) {
    let t51049 = t136 * t3297 * t50964;
    let t51051 = t2403 * t4772;
    let t51053 = t699 * t14792;
    let t51056 = t136 * t1113 * t50929;
    let t51058 = 4.0_f64 / 9.0_f64 * t50826;
    let t51073 = 8.0_f64 / 27.0_f64 * t50919;
    let t51078 = t51058 - t50828 / 3.0_f64 + t50832 / 3.0_f64 - 28.0_f64 / 81.0_f64 * t50834 + 2.0_f64 / 9.0_f64 * t43727 - 2.0_f64 / 3.0_f64 * t43729 - 8.0_f64 / 27.0_f64 * t43748 - 10.0_f64 / 81.0_f64 * t43750 - 2.0_f64 / 9.0_f64 * t50897 - 8.0_f64 * t50900 - 4.0_f64 / 3.0_f64 * t50903 - 2.0_f64 / 3.0_f64 * t50905 - 2.0_f64 * t50907 + 10.0_f64 / 9.0_f64 * t50912 + 40.0_f64 / 9.0_f64 * t50917 - t51073 - 10.0_f64 / 27.0_f64 * t50921 - 80.0_f64 / 81.0_f64 * t50926 + 2.0_f64 * t50931 + 2.0_f64 * t50934;
    (t51049, t51051, t51053, t51056, t51078)
}
