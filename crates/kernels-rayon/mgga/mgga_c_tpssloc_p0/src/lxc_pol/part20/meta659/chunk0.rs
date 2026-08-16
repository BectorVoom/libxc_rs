//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2452/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2452(t1020: f64, t1616: f64, t248: f64, t43216: f64, t10489: f64, t4644: f64, t10898: f64, t4630: f64, t10882: f64, t48569: f64, t10463: f64, t10493: f64, t10517: f64, t10886: f64, t10891: f64, t10937: f64, t10972: f64, t13762: f64, t14080: f64, t14099: f64, t1618: f64, t3098: f64, t42496: f64, t42653: f64, t43186: f64, t4579: f64, t4652: f64) -> f64 {
    let t50181 = t1020 * t248 * t43216 * t1616;
    let t50183 = t4644 * t10489;
    let t50189 = t10898 * t4630;
    let t50193 = t48569 * t10882;
    let t50207 = t10891 * t14099 / 96.0_f64 + t50181 / 10368.0_f64 - t50183 / 1152.0_f64 + t4644 * t10493 / 768.0_f64 + 19.0_f64 / 576.0_f64 * t42653 * t1618 - t50189 / 144.0_f64 + 19.0_f64 / 576.0_f64 * t10517 * t4652 + t50193 * t10886 / 3072.0_f64 + t4644 * t10463 / 4608.0_f64 + 5.0_f64 / 5184.0_f64 * t4644 * t10972 + t14080 * t3098 / 144.0_f64 - t42496 * t4579 / 144.0_f64 - t10937 * t13762 / 144.0_f64 + t43186 / 1152.0_f64;
    t50207
}
