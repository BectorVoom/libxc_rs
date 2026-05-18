//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 835/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk835<F: Float>(t1841: F, t35440: F, t44777: F, t11657: F, t2554: F, t7064: F, t35385: F, t883: F, t2932: F, t9647: F, t11680: F, t40820: F) -> (F, F, F, F, F) {
    let t44780 = F::new(0.10254034973522965711e-1) * t1841 * t35440 * t44777;
    let t44785 = t7064 * t11657 * t2554;
    let t44786 = F::new(0.32043859292259267849e-3) * t44785;
    let t44787 = t883 * t35385;
    let t44789 = t9647 * t2932 * t44787;
    let t44790 = F::new(0.64087718584518535698e-3) * t44789;
    let t44792 = t7064 * t11680 * t40820;
    (t44780, t44786, t44787, t44790, t44792)
}
