//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1449/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1449<F: Float>(t12213: F, t12255: F, t12294: F, t1897: F, t1935: F, t2580: F, t32147: F, t32149: F, t32152: F, t32154: F, t32159: F, t32161: F, t32167: F, t32169: F, t32172: F, t32185: F, t3727: F, t5397: F, t5836: F, t7129: F) -> F {
    let t39375 = F::new(0.30762104920568897134e-1) * t7129 * t12294 - t32147 + t32149 + t32152 + t32154 + t32159 + t32161 + F::new(0.46143157380853345702e-1) * t1897 * t12255 * t5836 - F::new(0.30762104920568897134e-1) * t1897 * t2580 * t12213 * t5397 - t32167 + t32169 + t32172 - F::new(0.76905262301422242837e-2) * t1935 * t3727 - t32185;
    t39375
}
