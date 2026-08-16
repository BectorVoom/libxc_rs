//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1261/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1261(t32514: f64, t7572: f64, t7573: f64, t10782: f64, t13063: f64, t1445: f64, t1835: f64, t1969: f64, t1998: f64, t32785: f64, t32791: f64, t32796: f64, t32806: f64, t32813: f64, t32815: f64, t32818: f64, t32821: f64, t32824: f64, t32827: f64, t32829: f64, t32832: f64, t5241: f64, t5640: f64, t7584: f64, t7585: f64) -> f64 {
    let t32835 = 0.12423108009070322895e3_f64 * t7572 * t7573 * t32514;
    let t32836 = -t32785 - 0.23005755572352449806e1_f64 * t1998 * t1445 * t10782 * t1835 - t32791 + 0.30674340763136599742e1_f64 * t5640 * t5241 * t13063 * t1969 - 0.23005755572352449806e2_f64 * t7584 * t7585 * t32796 + 0.13803453343411469884e2_f64 * t7572 * t7573 * t32796 - t32806 + t32813 + t32815 - t32818 - t32821 + t32824 + t32827 - t32829 - t32832 + t32835;
    t32836
}
