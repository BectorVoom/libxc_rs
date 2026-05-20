//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1123/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1123<F: Float>(t1312: F, t25832: F, t7235: F, t7313: F, t2322: F, t7003: F, t18163: F, t1937: F, t4254: F, t6993: F, t7239: F, t508: F) -> (F, F, F, F, F, F, F) {
    let t25834 = F::new(2.0) * t1312 * t25832;
    let t25838 = F::new(2.0) * t7235 * t7313;
    let t25840 = F::new(4.0) * t2322 * t7003;
    let t25842 = F::new(2.0) * t18163 * t1937;
    let t25844 = F::new(4.0) * t4254 * t6993;
    let t25846 = F::new(6.0) * t7235 * t7239;
    let t25851 = t508 * t25832;
    (t25834, t25838, t25840, t25842, t25844, t25846, t25851)
}
