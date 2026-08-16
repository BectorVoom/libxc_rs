//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2032/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2032(t531: f64, t7939: f64, t12550: f64, t12557: f64, t1442: f64, t15857: f64, t15904: f64, t1983: f64, t2036: f64, t22574: f64, t22584: f64, t22596: f64, t2314: f64, t2363: f64, t23938: f64, t24176: f64, t24428: f64, t24432: f64, t26161: f64, t26558: f64, t26905: f64, t26977: f64, t27219: f64, t33899: f64, t3929: f64, t4073: f64, t5107: f64, t56120: f64, t56194: f64, t652: f64, t7040: f64, t7042: f64, t7056: f64, t7685: f64, t7687: f64, t7890: f64, t7900: f64, t84347: f64, t90437: f64) -> f64 {
    let t93966 = t531 * t7939;
    let t93978 = 3.0_f64 * t1983 * t26905 * t22584 - t1442 * t24428 - t2036 * t15857 - 2.0_f64 * t7042 * t12557 + t7900 * t3929 - 3.0_f64 * t22574 * t24432 * t56120 - 4.0_f64 * t23938 * t4073 - 4.0_f64 * t26977 * t4073 - 4.0_f64 * t7042 * t12550 + 2.0_f64 * t26161 * t26558 * t90437 - 2.0_f64 * t652 * t7890 * t2363 - 4.0_f64 * t652 * t5107 * t7056 - 4.0_f64 * t2314 * t27219 - 6.0_f64 * t22574 * t24432 * t56194 + 6.0_f64 * t7685 * t24176 + 6.0_f64 * t1983 * t93966 * t22596 + 3.0_f64 * t1983 * t84347 * t7687 - 2.0_f64 * t7040 * t5107 - 6.0_f64 * t22574 * t33899 * t15904;
    t93978
}
