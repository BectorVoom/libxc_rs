//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1164/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1164<F: Float>(t1882: F, t36236: F, t25188: F, t28847: F, t36118: F, t36215: F, t36127: F, t8392: F, t36168: F, t10703: F, t1255: F, t143858: F, t1901: F, t2347: F, t25253: F, t2862: F, t28854: F, t29128: F, t29129: F, t29193: F, t296: F, t33835: F, t34012: F, t34181: F, t36121: F, t36218: F, t3886: F, t4129: F, t4167: F, t4246: F, t4311: F, t446: F, t56819: F, t6278: F, t684: F, t7105: F, t7131: F, t7584: F, t7611: F, t7672: F, t7679: F, t840: F, t871: F, t99238: F) -> (F, F) {
    let t154270 = t1882 * t36236;
    let t154285 = t25188 * t28847;
    let t154302 = t1882 * t36118;
    let t154304 = t1882 * t36215;
    let t154310 = t8392 * t36127;
    let t154327 = t1882 * t36168;
    let t154337 = F::new(2.0) / F::new(3.0) * t154270 + t446 * t840 * t871 * t7679 * t4129 / F::new(3.0) + F::new(4.0) / F::new(3.0) * t446 * t2862 * t7131 * t6278 + t143858 + F::new(2.0) / F::new(3.0) * t446 * t840 * t25253 * t7105 + F::new(4.0) / F::new(3.0) * t446 * t296 * t154285 - t446 * t840 * t4311 * t7611 / F::new(3.0) - F::new(4.0) / F::new(27.0) * t1901 * t56819 * t7672 * t2347 * t3886 - t1901 * t10703 * t36121 * t684 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t154302 - F::new(2.0) / F::new(9.0) * t154304 - F::new(4.0) * t1901 * t29128 * t29129 * t28854 - t154310 / F::new(27.0) + t446 * t840 * t34012 * t4167 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t446 * t2862 * t4246 * t34181 + F::new(4.0) / F::new(3.0) * t446 * t2862 * t1255 * t33835 - F::new(2.0) / F::new(9.0) * t1901 * t99238 * t29193 + t154327 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t1901 * t10703 * t36218 * t684 + F::new(2.0) / F::new(3.0) * t446 * t2862 * t4311 * t7584;
    (t154285, t154337)
}
