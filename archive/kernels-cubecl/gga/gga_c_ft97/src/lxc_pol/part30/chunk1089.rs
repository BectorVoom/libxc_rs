//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1089/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1089<F: Float>(t36092: F, t92: F, t25462: F, t36109: F, t317: F, t35972: F, t1253: F, t7611: F, t10683: F, t142941: F, t1506: F, t2347: F, t2360: F, t25412: F, t2665: F, t28496: F, t28938: F, t28944: F, t28997: F, t29008: F, t29026: F, t33808: F, t33996: F, t34008: F, t3886: F, t4162: F, t4255: F, t6216: F, t6217: F, t6219: F, t684: F) -> F {
    let t152547 = t36092 * t92;
    let t152558 = t25462 * t36109;
    let t152560 = t35972 * t317;
    let t152565 = t7611 * t1253;
    let t152574 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t6216 * t28938 * t1506 * t2360 * t3886 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t6216 * t28944 * t1506 * t2347 * t3886 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t6216 * t25412 * t142941 * t4255 - t33808 * t28997 / F::cast_from(18.0_f64) - t152547 * t6219 / F::cast_from(18.0_f64) + F::cast_from(2.0_f64) * t6216 * t10683 * t33996 * t4162 - t33808 * t29026 / F::cast_from(18.0_f64) + t29008 * t34008 / F::cast_from(9.0_f64) + t152558 / F::cast_from(27.0_f64) - t6216 * t2665 * t152560 * t684 / F::cast_from(18.0_f64) - t6216 * t2665 * t152565 * t684 / F::cast_from(18.0_f64) + F::cast_from(2.0_f64) * t6216 * t10683 * t6217 * t28496;
    t152574
}
