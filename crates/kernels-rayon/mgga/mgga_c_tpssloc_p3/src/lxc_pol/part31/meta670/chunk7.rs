//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1996/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1996(t100646: f64, t100659: f64, t100689: f64, t100718: f64, t100734: f64, t100747: f64, t100759: f64, t100772: f64, t100780: f64, t1877: f64, t2057: f64, t24191: f64, t24339: f64, t2522: f64, t25934: f64, t25945: f64, t26744: f64, t26756: f64, t28764: f64, t28789: f64, t28792: f64, t4314: f64, t5966: f64, t7110: f64, t7114: f64, t84800: f64) -> f64 {
    let t102087 = t1877 * t7110 * t5966 / 2.0_f64 - t1877 * t7114 * t100646 / 2.0_f64 - 3.0_f64 * t24191 * t100780 - t1877 * t24339 * t28792 - t1877 * t7114 * t100772 / 2.0_f64 + 3.0_f64 * t4314 * t2057 * t100759 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t100747 + 3.0_f64 * t4314 * t7110 * t28764 + 2.0_f64 * t26756 * t100689 + 2.0_f64 * t26756 * t100659 + t1877 * t84800 * t28789 - t1877 * t7114 * t100734 / 2.0_f64 + 3.0_f64 * t2522 * t2057 * t100718 - t1877 * t26744 * t25945 - t1877 * t26744 * t25934;
    t102087
}
