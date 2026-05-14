//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1031/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1031<F: Float>(t11438: F, t19686: F, t3021: F, t11442: F, t19671: F, t11322: F, t611: F, t9386: F, t11483: F, t11485: F, t1846: F, t34638: F, t34641: F, t34644: F, t34647: F, t34651: F, t34654: F, t34658: F) -> (F,) {
    let t34661 = t11438 * t3021 * t19686;
    let t34663 = t19671 * t11442;
    let t34666 = t611 * t9386 * t11322;
    let t34669 = t1846 * t11483 * t11485;
    let t34671 = -0.21720231316129303386e-4 * t34638 - 0.35979010468099443629e-7 * t34641 + 0.53968515702149165444e-6 * t34644 + 0.4797801045921060808e-7 * t34647 + 0.49166375783284505216e-8 * t34651 + 0.24583187891642252608e-8 * t34654 - 0.32777583855523003478e-8 * t34658 - 0.10860115658064651693e-4 * t34661 - 0.5686343261418565457e-6 * t34663 + 0.27462095132499841011e-4 * t34666 + 0.2318836277704281739e-4 * t34669;
    (t34671,)
}
