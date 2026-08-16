//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1241/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1241(t2036: f64, t2040: f64, t2075: f64, t2114: f64, t2165: f64, t27863: f64, t32674: f64, t33345: f64, t33360: f64, t33361: f64, t33364: f64, t33365: f64, t33367: f64, t33690: f64, t7266: f64, t7787: f64, t7796: f64, t7890: f64, t7983: f64, t8103: f64) -> f64 {
    let t34115 = -t2036 * t8103 - 2.0_f64 * t2040 * t27863 - 2.0_f64 * t2040 * t33690 - t2075 * t7983 - t2114 * t7890 - t2165 * t7787 - 2.0_f64 * t7266 * t7796 - t32674 - t33345 - t33360 - t33361 + t33364 + t33365 - t33367;
    t34115
}
