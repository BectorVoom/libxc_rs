//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2867/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2867<F: Float>(t1222: F, t3688: F, t697: F, t1226: F, t2438: F, t3566: F, t3781: F, t5330: F, t3362: F, t404: F, t3700: F, t43813: F) -> (F, F, F, F, F, F, F) {
    let t44925 = t1222 * t697 * t3688;
    let t44931 = t1222 * t2438 * t1226;
    let t44951 = t3566 * t3781;
    let t44952 = t44951 * t5330;
    let t44958 = F::new(1.0) / t404 / t3362;
    let t44980 = t1222 * t697 * t3700;
    let t45000 = F::cast_from(0.18467901234567901234e0_f64) * t43813;
    (t44925, t44931, t44951, t44952, t44958, t44980, t45000)
}
