//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1056/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1056<F: Float>(t11725: F, t2469: F, t2470: F, t24915: F, t2822: F, t3268: F, t33103: F, t33105: F, t33110: F, t33113: F, t33114: F, t33116: F, t33121: F, t33129: F, t3746: F, t3795: F, t7053: F, t7056: F, t7063: F, t972: F) -> F {
    let t33137 = F::cast_from(2.0_f64) * t2469 * t2822 * t3795 - F::cast_from(6.0_f64) * t2822 * t3746 * t7063 + F::cast_from(4.0_f64) * t11725 * t7056 + F::cast_from(2.0_f64) * t2470 * t33129 + F::cast_from(8.0_f64) * t24915 * t3268 - F::cast_from(2.0_f64) * t33121 * t972 - t3795 * t7053 - t33103 + t33105 + t33110 + t33113 + t33114 - t33116;
    t33137
}
