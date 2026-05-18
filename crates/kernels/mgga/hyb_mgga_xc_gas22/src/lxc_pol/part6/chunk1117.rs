//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1117/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1117<F: Float>(t10928: F, t10930: F, t10935: F, t10939: F, t10942: F, t10946: F, t10950: F, t7037: F, t7123: F, t9159: F, t9171: F, t9172: F) -> F {
    let t11002 = F::new(0.82524375e-1) * t10928 + F::new(0.16504875e0) * t10930 - t7123 + F::new(0.27595e0) * t7037 + F::new(0.5519e0) * t9159 - t9171 - t9172 - F::new(0.16557e0) * t10935 + F::new(0.49671e0) * t10939 - F::new(0.16557e0) * t10942 + F::new(0.248355e0) * t10946 + F::new(0.248355e0) * t10950;
    t11002
}
