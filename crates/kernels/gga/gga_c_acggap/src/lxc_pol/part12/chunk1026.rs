//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1026/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1026<F: Float>(t34609: F, t34618: F, t34620: F, t34626: F, t34632: F, t34650: F, t30659: F, t34611: F, t34614: F, t34616: F, t34622: F, t34630: F, t34636: F, t34638: F, t34640: F, t34644: F, t34647: F) -> (F,) {
    let t37175 = 11.0 / 96.0 * t34609;
    let t37179 = 0.2264262644851498949e-1 * t34618;
    let t37180 = 0.37737710747524982482e-2 * t34620;
    let t37182 = 0.18868855373762491241e-2 * t34626;
    let t37184 = 0.37737710747524982482e-1 * t34632;
    let t37190 = 0.22921875e-1 * t34650;
    let t37192 = -t37175 - 0.94344276868812456208e-2 * t34611 + 0.42874018118069736972e-2 * t34614 - 0.37737710747524982483e-1 * t34616 - t37179 + t37180 - 0.75475421495049964966e-2 * t34622 - t37182 + 0.12862205435420921092e-1 * t34630 - t37184 - 0.94344276868812456207e-3 * t34636 + 0.31448092289604152069e-3 * t34638 + 0.56606566121287473722e-1 * t34640 - 0.94344276868812456204e-2 * t34644 + 0.1528125e-1 * t34647 + t37190 + 0.51448821741683684367e-2 * t30659;
    (t37192,)
}
