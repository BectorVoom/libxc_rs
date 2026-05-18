//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1395/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1395<F: Float>(t20535: F, t34688: F, t6578: F, t12881: F, t4382: F, t544: F, t874: F, t27114: F, t901: F, t30843: F, t12963: F, t1339: F, t1537: F, t1540: F, t30835: F, t34659: F, t34662: F, t34665: F, t34668: F, t34672: F, t34675: F, t34678: F, t34681: F, t34684: F, t34687: F) -> F {
    let t34690 = t20535 * t34688 * t6578;
    let t34691 = F::new(0.11502877786176224903e1) * t34690;
    let t34699 = F::new(0.53625734927775640005e1) * t544 * t4382 * t874 * t12881;
    let t34700 = t27114 * t901;
    let t34701 = F::new(0.14896037479937677779e-1) * t34700;
    let t34702 = F::new(0.63904876589867916128e-1) * t30843;
    let t34703 = -t34659 - t34662 + t34665 + t34668 - t34672 + t34675 - t34678 + t34681 + t34684 + t34687 + t34691 - F::new(0.51123901271894332902e1) * t1537 * t1339 * t12963 * t1540 - t34699 + t34701 + t30835 + t34702;
    t34703
}
