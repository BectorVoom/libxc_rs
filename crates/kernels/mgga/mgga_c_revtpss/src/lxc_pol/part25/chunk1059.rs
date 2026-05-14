//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1059/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1059<F: Float>(t18163: F, t1937: F, t4254: F, t6993: F, t7235: F, t7239: F, t25832: F, t508: F, t651: F, t1936: F, t3813: F, t7003: F, t1310: F, t7002: F, t2033: F, t530: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t25842 = 2.0 * t18163 * t1937;
    let t25844 = 4.0 * t4254 * t6993;
    let t25846 = 6.0 * t7235 * t7239;
    let t25851 = t508 * t25832;
    let t25853 = 2.0 * t651 * t25851;
    let t25856 = t3813 * t1936;
    let t25858 = 2.0 * t651 * t25856;
    let t25860 = 4.0 * t4254 * t7003;
    let t25861 = t1310 * t7002;
    let t25863 = 4.0 * t651 * t25861;
    let t25864 = t530 * t2033;
    (t25842, t25844, t25846, t25851, t25853, t25856, t25858, t25860, t25861, t25863, t25864)
}
