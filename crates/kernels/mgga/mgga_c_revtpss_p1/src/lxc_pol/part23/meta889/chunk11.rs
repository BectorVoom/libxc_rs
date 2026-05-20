//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2830/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2830<F: Float>(t1558: F, t5966: F, t14785: F, t14786: F, t14791: F, t23160: F, t23279: F, t2745: F, t2749: F, t40425: F, t4362: F, t51014: F, t6022: F, t61620: F, t61623: F, t61628: F, t61630: F, t61632: F, t61641: F, t61645: F, t61660: F, t61669: F, t61673: F, t61675: F, t61677: F, t76302: F, t837: F) -> (F, F) {
    let t76474 = t5966 * t1558;
    let t76493 = F::cast_from(0.21437009059034868486e-4_f64) * t61620 - F::cast_from(0.8131200449485652516e-3_f64) * t61623 + F::cast_from(0.17149607247227894789e-3_f64) * t61628 + F::cast_from(0.36014175219178579057e0_f64) * t61630 - F::cast_from(0.12004725073059526352e0_f64) * t61632 - F::cast_from(0.45738002528356795403e-2_f64) * t61641 - F::cast_from(0.12862205435420921092e-1_f64) * t2745 * t14785 * t76302 * t2749 - F::cast_from(0.51448821741683684366e-2_f64) * t4362 * t14791 * t23160 * t14786 - F::cast_from(0.12846167376791569079e-2_f64) * t40425 + F::cast_from(0.77173232612525526552e-1_f64) * t2745 * t51014 * t76474 * t2749 - F::cast_from(0.1543464652250510531e-1_f64) * t4362 * t14791 * t6022 * t14786 - F::cast_from(0.12862205435420921092e-1_f64) * t2745 * t14785 * t23279 * t837 + F::cast_from(0.18292914397043087774e-2_f64) * t61645 - F::cast_from(0.6002362536529763176e-1_f64) * t61660 + F::cast_from(0.42874018118069736972e-4_f64) * t61669 + F::cast_from(0.21437009059034868486e-4_f64) * t61673 + F::cast_from(0.16262400898971305032e-2_f64) * t61675 + F::cast_from(0.34013387707001991332e0_f64) * t61677;
    (t76474, t76493)
}
