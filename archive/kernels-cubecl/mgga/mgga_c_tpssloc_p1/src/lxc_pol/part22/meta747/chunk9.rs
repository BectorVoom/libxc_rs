//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2497/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2497<F: Float>(t21684: F, t225: F, t1066: F, t14529: F, t14555: F, t1635: F, t17575: F, t18061: F, t18062: F, t18074: F, t18166: F, t21663: F, t21692: F, t25757: F, t3169: F, t388: F, t4557: F, t4657: F, t4694: F, t50628: F, t5848: F, t5944: F, t61646: F, t63215: F) -> F {
    let t70987 = t21684 * t225;
    let t71015 = -F::cast_from(18.0_f64) * t18061 * t25757 * t50628 + F::cast_from(3.0_f64) * t388 * t4657 * t5848 - F::cast_from(3.0_f64) * t1066 * t70987 - F::cast_from(3.0_f64) * t14529 * t5944 - F::cast_from(3.0_f64) * t14555 * t5944 - F::cast_from(3.0_f64) * t1635 * t61646 - F::cast_from(3.0_f64) * t1635 * t63215 - F::cast_from(3.0_f64) * t17575 * t4694 + F::cast_from(6.0_f64) * t18062 * t4557 - F::cast_from(3.0_f64) * t18074 * t4694 - F::cast_from(3.0_f64) * t18166 * t4557 - t21663 * t3169 + F::cast_from(6.0_f64) * t21692 * t3169;
    t71015
}
