//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1396/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1396(t20535: f64, t34688: f64, t6578: f64, t12881: f64, t4382: f64, t544: f64, t874: f64, t27114: f64, t901: f64, t30843: f64, t12963: f64, t1339: f64, t1537: f64, t1540: f64, t30835: f64, t34659: f64, t34662: f64, t34665: f64, t34668: f64, t34672: f64, t34675: f64, t34678: f64, t34681: f64, t34684: f64, t34687: f64) -> f64 {
    let t34690 = t20535 * t34688 * t6578;
    let t34691 = 0.11502877786176224903e1_f64 * t34690;
    let t34699 = 0.53625734927775640005e1_f64 * t544 * t4382 * t874 * t12881;
    let t34700 = t27114 * t901;
    let t34701 = 0.14896037479937677779e-1_f64 * t34700;
    let t34702 = 0.63904876589867916128e-1_f64 * t30843;
    let t34703 = -t34659 - t34662 + t34665 + t34668 - t34672 + t34675 - t34678 + t34681 + t34684 + t34687 + t34691 - 0.51123901271894332902e1_f64 * t1537 * t1339 * t12963 * t1540 - t34699 + t34701 + t30835 + t34702;
    t34703
}
