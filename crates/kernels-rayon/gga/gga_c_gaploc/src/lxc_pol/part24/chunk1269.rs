//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1269/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1269(t32897: f64, t6066: f64, t6111: f64, t10811: f64, t7772: f64, t2976: f64, t7503: f64, t10820: f64, t818: f64, t825: f64, t22706: f64, t2684: f64) -> (f64, f64, f64, f64, f64) {
    let t32900 = 0.85801175884441024006e1_f64 * t6111 * t6066 * t32897;
    let t32902 = 0.17875244975925213335e2_f64 * t10811 * t7772;
    let t32903 = t2976 * t7503;
    let t32904 = 0.89376224879626066674e-1_f64 * t32903;
    let t32907 = 0.24539472610509279794e2_f64 * t825 * t818 * t10820;
    let t32910 = 0.11656249489991907902e3_f64 * t2684 * t22706 * t10820;
    (t32900, t32902, t32904, t32907, t32910)
}
