//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1220/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1220(t113135: f64, t118376: f64, t118381: f64, t118436: f64, t118465: f64, t118949: f64, t119737: f64, t119743: f64, t119746: f64, t119755: f64, t119763: f64, t119766: f64, t119780: f64, t1649: f64, t1877: f64, t22959: f64, t23290: f64, t2522: f64, t25372: f64, t25892: f64, t25901: f64, t25905: f64, t25928: f64, t25934: f64, t25938: f64, t25945: f64, t28: f64, t30753: f64, t30757: f64, t30770: f64, t32886: f64, t33065: f64, t6670: f64, t6841: f64, t8366: f64) -> f64 {
    let t119783 = -t1877 * t30757 * t25934 / 2.0_f64 - t1877 * t6670 * t119737 + t1877 * t30770 * t25945 + t118436 * t25928 + 2.0_f64 * t25372 * t119743 - t1877 * t6670 * t119746 + t1877 * t30753 * t1649 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t32886 * t6841 - 3.0_f64 * t118376 * t119755 + 3.0_f64 / 2.0_f64 * t2522 * t8366 * t25901 + 3.0_f64 * t118381 * t25892 + 3.0_f64 * t113135 * t119763 - t118465 - t1877 * t6670 * t119766 - t1877 * t23290 * t33065 + 3.0_f64 / 2.0_f64 * t2522 * t8366 * t25905 + t1877 * t118949 * t28 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t8366 * t25938 - 3.0_f64 * t22959 * t119780;
    t119783
}
