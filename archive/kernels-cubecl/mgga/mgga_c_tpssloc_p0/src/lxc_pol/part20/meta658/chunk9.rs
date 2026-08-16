//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2449/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2449<F: Float>(t10250: F, t1041: F, t10884: F, t14172: F, t14184: F, t1607: F, t1616: F, t1618: F, t3048: F, t3070: F, t3071: F, t3117: F, t42358: F, t42554: F, t42756: F, t43167: F, t4582: F, t4593: F, t48554: F, t50078: F, t50084: F, t50094: F, t50098: F, t50100: F) -> F {
    let t50102 = F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t3117 * t14184 - F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t1041 * t4582 * t14172 * t48554 + t42756 * t1618 / F::cast_from(3072.0_f64) + t50078 - t3070 * t3071 * t1616 * t10250 / F::cast_from(768.0_f64) - t50084 / F::cast_from(1152.0_f64) + t43167 / F::cast_from(768.0_f64) - t42358 * t4582 * t4593 * t10884 / F::cast_from(3072.0_f64) - F::cast_from(5.0_f64) / F::cast_from(864.0_f64) * t3048 * t14184 + t50094 / F::cast_from(1152.0_f64) - F::cast_from(77.0_f64) / F::cast_from(486.0_f64) * t42554 * t1607 + F::cast_from(11.0_f64) / F::cast_from(324.0_f64) * t50098 + t50100 / F::cast_from(144.0_f64);
    t50102
}
