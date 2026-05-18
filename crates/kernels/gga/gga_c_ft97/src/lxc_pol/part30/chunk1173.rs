//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1173/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1173<F: Float>(t10703: F, t1248: F, t143660: F, t144197: F, t144199: F, t144212: F, t144219: F, t144227: F, t144236: F, t1476: F, t15312: F, t15369: F, t154787: F, t154794: F, t154807: F, t154813: F, t154820: F, t154827: F, t154833: F, t1901: F, t2360: F, t24898: F, t28843: F, t29245: F, t296: F, t33953: F, t3886: F, t4255: F, t4260: F, t446: F, t6393: F, t7021: F, t7672: F, t840: F, t871: F) -> F {
    let t154837 = t144197 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t144199 - F::new(2.0) / F::new(9.0) * t154787 - F::new(2.0) / F::new(3.0) * t446 * t840 * t6393 * t7021 - t446 * t296 * t154794 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t1901 * t15312 * t143660 * t4260 - t144212 + t446 * t840 * t871 * t33953 * t1248 / F::new(3.0) - t144219 - F::new(4.0) / F::new(9.0) * t154807 - F::new(2.0) / F::new(3.0) * t446 * t840 * t28843 * t1476 - F::new(4.0) / F::new(9.0) * t154813 + F::new(4.0) / F::new(9.0) * t1901 * t15312 * t7672 * t2360 * t3886 + F::new(2.0) / F::new(27.0) * t154820 - F::new(4.0) / F::new(3.0) * t1901 * t15369 * t24898 * t29245 - F::new(2.0) / F::new(9.0) * t144227 + t154827 / F::new(27.0) + t144236 - t1901 * t10703 * t143660 * t4255 / F::new(9.0) - F::new(2.0) / F::new(3.0) * t446 * t296 * t154833;
    t154837
}
