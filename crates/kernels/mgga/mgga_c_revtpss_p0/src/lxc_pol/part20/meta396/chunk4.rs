//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1461/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1461<F: Float>(t41308: F, t41312: F, t41320: F, t41327: F, t41330: F, t41332: F, t41334: F, t41336: F, t41365: F, t41367: F, t41433: F, t41436: F, t41439: F, t41441: F) -> F {
    let t41652 = -F::cast_from(0.23917333333333333334e1_f64) * t41365 + F::cast_from(0.79724444444444444444e0_f64) * t41367 + F::cast_from(0.23917333333333333333e1_f64) * t41308 + F::new(0.71752e1) * t41312 + F::new(0.17938e1) * t41320 - F::cast_from(0.59793333333333333333e0_f64) * t41327 - F::cast_from(0.79724444444444444446e0_f64) * t41330 - F::cast_from(0.5314962962962962963e0_f64) * t41332 + F::cast_from(0.39862222222222222223e0_f64) * t41334 + F::cast_from(0.44291358024691358024e0_f64) * t41336 - F::cast_from(0.82156666666666666668e-1_f64) * t41433 + F::new(0.197176e1) * t41436 + F::cast_from(0.49293999999999999999e0_f64) * t41439 + F::cast_from(0.97370864197530864199e0_f64) * t41441;
    t41652
}
