//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1150/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1150<F: Float>(t2061: F, t7063: F, t25410: F, t25413: F, t120111: F, t120114: F, t120117: F, t120132: F, t119823: F, t121817: F, t121913: F, t32474: F) -> (F, F, F, F, F, F, F, F, F) {
    let t122002 = t7063 * t2061;
    let t122003 = t122002 * t25410;
    let t122004 = t122003 * t25413;
    let t122008 = F::new(0.7437465841810202164e-5) * t120111;
    let t122009 = F::new(0.39671442800215618342e-4) * t120114;
    let t122010 = F::new(0.47023883532522246276e-4) * t120117;
    let t122015 = F::new(0.26773803678175077507e-4) * t120132;
    let t122024 = t119823 * t121817;
    let t122026 = t32474 * t121913;
    (t122002, t122003, t122004, t122008, t122009, t122010, t122015, t122024, t122026)
}
