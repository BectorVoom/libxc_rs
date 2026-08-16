//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2238/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2238<F: Float>(t27708: F, t3336: F, t11108: F, t7840: F, t100425: F, t100471: F, t100513: F, t100560: F, t100606: F, t100650: F, t100696: F, t100748: F, t100794: F, t1100: F, t1102: F, t16612: F, t1699: F, t198: F, t25709: F, t25713: F, t27712: F, t27717: F, t3329: F, t3333: F, t336: F, t5019: F, t5023: F, t63827: F, t7181: F, t94138: F, t94142: F, t94149: F, t99618: F, t99673: F, t99728: F, t99790: F, t99847: F, t99901: F, t99950: F) -> F {
    let t100802 = t27708 * t3336;
    let t100806 = t7840 * t11108;
    let t100833 = t198 * t336 * (t99618 + t99673 + t99728 + t99790 + t99847 + t99901 + t99950 + t100425 + t100471 + t100513 + t100560 + t100606 + t100650 + t100696 + t100748 + t100794) * t1102 - F::cast_from(2.0_f64) * t5023 * t100802 * t1100 + F::cast_from(2.0_f64) * t5023 * t100806 * t3333 - t5023 * t27712 * t3329 - t5023 * t94138 * t1699 + F::cast_from(4.0_f64) * t5023 * t94142 * t27717 - F::cast_from(2.0_f64) * t5023 * t25709 * t5019 - F::cast_from(6.0_f64) * t5023 * t94149 * t1699 * t3333 + F::cast_from(4.0_f64) * t5023 * t25713 * t63827 + F::cast_from(2.0_f64) * t5023 * t25713 * t1699 * t3329 - t5023 * t7181 * t16612;
    t100833
}
