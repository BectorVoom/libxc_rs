//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 693/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk693<F: Float>(t4820: F, t6510: F, t107: F, t2299: F, t1415: F, t1359: F, t2405: F, t544: F, t6520: F, t4376: F, t901: F, t1328: F, t6508: F, t2365: F, t4391: F, t4625: F, t914: F) -> (F, F, F, F, F, F, F, F) {
    let t6825 = t4820 * t6510;
    let t6830 = t2299 * t107;
    let t6831 = t1415 * t6830;
    let t6834 = t1359 * t2405;
    let t6835 = t544 * t6834;
    let t6838 = t4820 * t6520;
    let t6841 = t4376 * t901;
    let t6843 = t6508 * t1328;
    let t6844 = t2365 * t6843;
    let t6845 = t4391 * t6844;
    let t6847 = t4625 * t914;
    (t6825, t6831, t6834, t6835, t6838, t6841, t6845, t6847)
}
