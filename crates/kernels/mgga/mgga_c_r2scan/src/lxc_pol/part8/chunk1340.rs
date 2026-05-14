//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1340/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1340<F: Float>(t19827: F, t24171: F, t24178: F, t24447: F, t24453: F, t2551: F, t2562: F, t2573: F, t2670: F, t28273: F, t28276: F, t28292: F, t28296: F, t28301: F, t32444: F, t32485: F, t360: F, t5108: F, t5109: F, t7383: F, t7461: F, t9115: F, t9194: F, t9312: F) -> (F,) {
    let t32870 = -0.15602799132097683414e1 * t7461 * t360 * t2562 * t9115 - 0.69345773920434148506e0 * t28273 - 0.10401866088065122276e1 * t28276 - 0.20803732176130244552e1 * t28292 - 0.41607464352260489104e1 * t28296 + 0.31205598264195366828e1 * t7383 * t9194 - 0.97574405393827830185e-3 * t28301 + t24171 + t24178 + 0.15602799132097683414e1 * t19827 * t5109 * t32444 * t2573 - 0.39006997830244208535e0 * t5108 * t5109 * t32485 * t2551 - 0.39006997830244208535e0 * t2670 * t9312 + 0.86743646395112941038e-3 * t24447 + t24453;
    (t32870,)
}
