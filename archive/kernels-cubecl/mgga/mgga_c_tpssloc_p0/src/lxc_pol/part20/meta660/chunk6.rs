//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2470/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2470<F: Float>(t10160: F, t10182: F, t1049: F, t1052: F, t1065: F, t11085: F, t13736: F, t13939: F, t14526: F, t14545: F, t14555: F, t14658: F, t1635: F, t3026: F, t3169: F, t3174: F, t3176: F, t3206: F, t349: F, t388: F, t43440: F, t43619: F, t4557: F, t4693: F, t4694: F, t50457: F, t990: F) -> F {
    let t50744 = F::cast_from(6.0_f64) * t1052 * t1065 * t14658 * t3174 + F::cast_from(6.0_f64) * t1052 * t3174 * t3206 * t4693 + F::cast_from(3.0_f64) * t1049 * t13939 * t388 + F::cast_from(3.0_f64) * t14526 * t388 * t990 + t349 * t388 * t50457 - F::cast_from(6.0_f64) * t10160 * t4694 + F::cast_from(6.0_f64) * t10182 * t4557 - t11085 * t4557 - F::cast_from(18.0_f64) * t13736 * t3026 - F::cast_from(18.0_f64) * t13736 * t3169 + F::cast_from(6.0_f64) * t14545 * t3176 + F::cast_from(6.0_f64) * t14555 * t3176 - t1635 * t43440 - t1635 * t43619;
    t50744
}
