//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1113/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1113<F: Float>(t34610: F, t34612: F, t34617: F, t34618: F, t34621: F, t34623: F, t34627: F, t34633: F, t34636: F, t34638: F, t34640: F, t37190: F, t39402: F, t39406: F, t39412: F, t39414: F, t39418: F, t39422: F) -> F {
    let t39424 = -t34610 + F::new(0.114609375e-1) * t39402 + F::new(0.12862205435420921092e-2) * t39406 - t34612 - t34617 - F::new(0.11321313224257494745e-1) * t34618 + t34621 - t34623 - t34627 - t34633 - F::new(0.94344276868812456204e-3) * t34636 + F::new(0.31448092289604152068e-3) * t34638 + F::new(0.28303283060643736861e-1) * t34640 - F::new(0.17149607247227894789e-2) * t39412 - F::new(0.17149607247227894789e-2) * t39414 + t37190 + F::new(0.94344276868812456204e-3) * t39418 - F::new(0.18868855373762491241e-2) * t39422;
    t39424
}
