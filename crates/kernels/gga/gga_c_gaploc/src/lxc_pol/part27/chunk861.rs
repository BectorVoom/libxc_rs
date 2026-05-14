//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 861/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk861<F: Float>(t10012: F, t723: F, t9438: F, t2684: F, t10007: F, t701: F, t2615: F, t2628: F, t2673: F, t7442: F, t787: F, t2563: F, t900: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10013 = t10012 * t723;
    let t10014 = t9438 * t10013;
    let t10015 = t2684 * t10014;
    let t10017 = t10007 * t701;
    let t10018 = t9438 * t10017;
    let t10019 = t2615 * t10018;
    let t10022 = 0.59584149919750711116e-1 * t2673 * t2628;
    let t10023 = t787 * t7442;
    let t10024 = t900 * t2563;
    (t10013, t10014, t10015, t10017, t10018, t10019, t10022, t10023, t10024)
}
