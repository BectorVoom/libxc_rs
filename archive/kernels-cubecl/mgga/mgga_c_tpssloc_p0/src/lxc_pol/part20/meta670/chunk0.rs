//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2517/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2517<F: Float>(t136: F, t3297: F, t50964: F, t2403: F, t4772: F, t14792: F, t699: F, t1113: F, t50929: F, t50826: F, t50919: F, t43727: F, t43729: F, t43748: F, t43750: F, t50828: F, t50832: F, t50834: F, t50897: F, t50900: F, t50903: F, t50905: F, t50907: F, t50912: F, t50917: F, t50921: F, t50926: F, t50931: F, t50934: F) -> (F, F, F, F, F) {
    let t51049 = t136 * t3297 * t50964;
    let t51051 = t2403 * t4772;
    let t51053 = t699 * t14792;
    let t51056 = t136 * t1113 * t50929;
    let t51058 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t50826;
    let t51073 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t50919;
    let t51078 = t51058 - t50828 / F::cast_from(3.0_f64) + t50832 / F::cast_from(3.0_f64) - F::cast_from(28.0_f64) / F::cast_from(81.0_f64) * t50834 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t43727 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t43729 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t43748 - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t43750 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t50897 - F::cast_from(8.0_f64) * t50900 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t50903 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t50905 - F::cast_from(2.0_f64) * t50907 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t50912 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t50917 - t51073 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t50921 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t50926 + F::cast_from(2.0_f64) * t50931 + F::cast_from(2.0_f64) * t50934;
    (t51049, t51051, t51053, t51056, t51078)
}
