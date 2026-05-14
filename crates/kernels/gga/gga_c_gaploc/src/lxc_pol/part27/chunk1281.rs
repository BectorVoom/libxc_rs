//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1281/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1281<F: Float>(t29233: F, t29242: F, t32104: F, t32106: F, t32110: F, t32117: F, t32119: F, t32123: F, t32125: F, t32128: F, t32131: F, t32135: F, t32139: F, t32143: F, t12213: F, t12255: F, t12294: F, t1897: F, t1935: F, t2580: F, t32147: F, t32149: F, t32152: F, t32154: F, t32159: F, t32161: F, t32167: F, t32169: F, t32172: F, t32185: F, t3727: F, t5397: F, t5836: F, t7129: F) -> (F, F) {
    let t39362 = t29233 - t29242 - t32104 + t32106 + t32110 - t32117 - t32119 - t32123 + t32125 + t32128 - t32131 - t32135 - t32139 - t32143;
    let t39375 = 0.30762104920568897134e-1 * t7129 * t12294 - t32147 + t32149 + t32152 + t32154 + t32159 + t32161 + 0.46143157380853345702e-1 * t1897 * t12255 * t5836 - 0.30762104920568897134e-1 * t1897 * t2580 * t12213 * t5397 - t32167 + t32169 + t32172 - 0.76905262301422242837e-2 * t1935 * t3727 - t32185;
    (t39362, t39375)
}
