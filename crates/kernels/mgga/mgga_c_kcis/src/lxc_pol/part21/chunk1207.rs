//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1207/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1207<F: Float>(t91794: F, t91796: F, t91799: F, t91801: F, t91804: F, t91806: F, t91809: F, t91811: F, t91814: F, t91816: F, t91818: F, t91820: F, t91822: F, t91825: F) -> F {
    let t92134 = -F::new(0.1125e1) * t91794 - F::new(0.5625e0) * t91796 - F::new(0.1125e1) * t91799 + F::new(0.97125e0) * t91801 - F::new(0.225e1) * t91804 - F::new(0.5625e0) * t91806 + F::new(0.1125e1) * t91809 + F::new(0.1125e1) * t91811 + F::new(0.809375e-1) * t91814 + F::new(0.2428125e0) * t91816 + F::new(0.1125e1) * t91818 - F::new(0.485625e1) * t91820 - F::new(0.3375e1) * t91822 - F::new(0.485625e0) * t91825;
    t92134
}
