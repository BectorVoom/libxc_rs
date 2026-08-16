//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 913/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk913(t1336: f64, t240: f64, t241: f64, t3787: f64, t22824: f64, t31159: f64, t22866: f64, t8462: f64, t2690: f64, t544: f64, t553: f64, t8467: f64) -> (f64, f64, f64, f64) {
    let t114016 = t1336 * t3787 * t240 * t241;
    let t114025 = t22824 * t31159;
    let t114027 = t22866 * t8462;
    let t114038 = t544 * t553 * t2690 * t8467;
    (t114016, t114025, t114027, t114038)
}
