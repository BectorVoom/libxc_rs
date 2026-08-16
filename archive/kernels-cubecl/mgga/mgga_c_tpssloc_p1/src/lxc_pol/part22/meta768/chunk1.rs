//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2603/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2603<F: Float>(t1174: F, t135: F, t22128: F, t22132: F, t11665: F, t1216: F, t1227: F, t15438: F, t15507: F, t18590: F, t18955: F, t19062: F, t19072: F, t21758: F, t22158: F, t3577: F, t45128: F, t4582: F, t4984: F, t5005: F, t5024: F, t50992: F, t52759: F, t65914: F, t65920: F, t65966: F, t70330: F) -> F {
    let t72597 = t1174 * t135 * t22128;
    let t72600 = t1174 * t135 * t22132;
    let t72622 = -t65966 * t4984 / F::cast_from(1024.0_f64) - t72597 / F::cast_from(864.0_f64) - t72600 / F::cast_from(144.0_f64) - t1227 * t4582 * t50992 * t70330 / F::cast_from(192.0_f64) + F::cast_from(5.0_f64) / F::cast_from(324.0_f64) * t5024 * t18955 - t15438 * t19062 / F::cast_from(1024.0_f64) + t52759 + t15507 * t19072 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t65914 - t5005 * t18590 / F::cast_from(384.0_f64) - t65920 / F::cast_from(1152.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t11665 * t22158 - F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t3577 * t45128 * t21758 * t1216;
    t72622
}
