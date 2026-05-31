//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1166/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1166<F: Float>(t1882: F, t36220: F, t36165: F, t8392: F, t55797: F, t7672: F, t10443: F, t10683: F, t10696: F, t10703: F, t1091: F, t112663: F, t112760: F, t113656: F, t114847: F, t11593: F, t1255: F, t143621: F, t143998: F, t144262: F, t152772: F, t15299: F, t15312: F, t15369: F, t15460: F, t1901: F, t2749: F, t2874: F, t29071: F, t29128: F, t29198: F, t29203: F, t29208: F, t296: F, t33830: F, t34197: F, t34202: F, t36060: F, t36112: F, t36126: F, t36186: F, t3746: F, t4162: F, t4167: F, t4181: F, t446: F, t6361: F, t684: F, t7679: F, t840: F) -> (F, F) {
    let t154399 = t1882 * t36220;
    let t154439 = t8392 * t36165;
    let t154463 = t55797 * t7672;
    let t154467 = -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t154399 - F::cast_from(2.0_f64) * t1901 * t29128 * t10696 * t7679 * t4181 + t1901 * t10443 * t36126 / F::cast_from(9.0_f64) + t1901 * t2874 * t144262 * t1091 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11593 * t2874 * t34202 * t3746 - F::cast_from(2.0_f64) * t446 * t10683 * t1255 * t33830 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t15299 * t152772 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t840 * t2749 * t36112 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t113656 * t6361 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t15369 * t34197 * t4162 + F::cast_from(2.0_f64) * t1901 * t15460 * t143621 * t4167 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t154439 - F::cast_from(2.0_f64) * t1901 * t29071 * t34202 * t4162 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t112760 * t29198 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t114847 * t29203 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t112663 * t29208 - t1901 * t10703 * t36186 * t684 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t15312 * t36060 * t684 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t143998 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t296 * t154463;
    (t154463, t154467)
}
