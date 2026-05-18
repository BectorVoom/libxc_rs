//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 709/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk709<F: Float>(t12645: F, t213: F, t1011: F, t1018: F, t12454: F, t12460: F, t12462: F, t12500: F, t12503: F, t12505: F, t12631: F, t12637: F, t12641: F, t12644: F, t139: F, t172: F, t175: F, t197: F, t198: F, t3194: F, t3203: F, t3209: F, t3213: F, t3220: F) -> F {
    let t12646 = t12645 * t213;
    let t12649 = F::new(0.74295e-1) * t12454 * t3209 + F::new(0.4953e-1) * t3194 * t3213 - F::new(0.619125e-2) * t12460 * t12462 - F::new(0.619125e-2) * t197 * t12500 + F::new(0.371475e-1) * t12503 * t12505 - F::new(0.23583209876543209876e-1) * t139 * t172 * t175 + F::new(0.619125e-2) * t12631 * t198 - F::new(0.1857375e-1) * t1011 * t3220 + F::new(0.619125e-2) * t12637 * t3203 - F::new(0.371475e-1) * t12641 * t1018 + F::new(0.41275e-2) * t12644 * t12646;
    t12649
}
