//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 547/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk547<F: Float>(t1006: F, t583: F, t1689: F, t1743: F, t2696: F, t2699: F, t2702: F, t2707: F, t203: F, t184: F, t221: F, t1755: F, t1756: F, t2760: F, t2763: F, t2766: F, t2770: F) -> (F, F, F, F, F, F, F) {
    let t2807 = t1006 * t583;
    let t2808 = F::new(4.0) / F::new(45.0) * t2807;
    let t2814 = -t1743 - F::new(0.62972222222222222223e-3) * t1689 - F::new(0.62972222222222222223e-3) * t2696 + F::new(0.12594444444444444445e-2) * t2699 - F::new(0.37783333333333333334e-2) * t2702 - F::new(0.37783333333333333334e-2) * t2707;
    let t2815 = t203 * t2814;
    let t2816 = t2815 * t184;
    let t2818 = F::new(2.0) / F::new(15.0) * t2816 * t221;
    let t2824 = -t1755 - F::new(0.62972222222222222223e-3) * t1756 - F::new(0.62972222222222222223e-3) * t2760 + F::new(0.12594444444444444445e-2) * t2763 - F::new(0.37783333333333333334e-2) * t2766 + F::new(0.37783333333333333334e-2) * t2770;
    (t2807, t2808, t2814, t2815, t2816, t2818, t2824)
}
