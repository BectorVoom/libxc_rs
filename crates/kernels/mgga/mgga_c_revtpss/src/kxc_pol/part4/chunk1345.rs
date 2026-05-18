//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1345/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1345<F: Float>(t12252: F, t12261: F, t12263: F, t12265: F, t12349: F, t12352: F, t16731: F, t16852: F, t16855: F, t16858: F, t16860: F, t16863: F, t16865: F, t16883: F, t16887: F, t16890: F, t16893: F, t16895: F, t16898: F, t16901: F, t16904: F, t16940: F) -> F {
    let t16942 = F::new(0.18257037037037037037e-1) * t12252 + F::new(0.18257037037037037037e0) * t12261 - F::new(0.54771111111111111111e-1) * t12263 - F::new(0.10954222222222222222e0) * t12265 + F::new(0.142419375e1) * t16852 - F::new(0.76790625e-1) * t16855 - F::new(0.1898925e1) * t16858 - F::new(0.9494625e0) * t16860 + F::new(0.3071625e0) * t16863 + F::new(0.15358125e0) * t16865 + t16883 - F::new(0.19931111111111111111e0) * t16731 + F::new(0.16431333333333333333e0) * t16887 + F::new(0.49293999999999999999e0) * t16890 - t16893 - F::new(0.54771111111111111112e-1) * t16895 - t12349 - t12352 - F::new(0.27385555555555555556e-1) * t16898 - F::new(0.16431333333333333333e0) * t16901 + F::new(0.32862666666666666666e0) * t16904 + t16940;
    t16942
}
