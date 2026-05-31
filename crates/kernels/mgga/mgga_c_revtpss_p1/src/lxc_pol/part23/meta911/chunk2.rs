//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2928/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2928<F: Float>(t77681: F, t77705: F, t77732: F, t77747: F, t77801: F, t77824: F, t77846: F, t77860: F, t11404: F, t15343: F, t19156: F, t19167: F, t23706: F, t23717: F, t23723: F, t41756: F, t41779: F, t4685: F, t4708: F, t52809: F, t52820: F, t6158: F, t6177: F, t6206: F, t77639: F, t77641: F, t77643: F, t77645: F, t77647: F, t77657: F, t965: F, t973: F) -> (F, F) {
    let t77863 = t77681 + t77705 + t77732 + t77747 + t77801 + t77824 + t77846 + t77860;
    let t77873 = -t77639 - t77641 - t77643 + t77645 - t77647 + F::cast_from(0.17544670867903938621e1_f64) * t19156 * t4708 + F::cast_from(0.17544670867903938621e1_f64) * t15343 * t6206 + F::cast_from(0.17544670867903938621e1_f64) * t4685 * t19167 - t77657 - F::cast_from(6.0_f64) * t52809 * t6158 + F::cast_from(6.0_f64) * t11404 * t23706 + F::cast_from(0.5848223622634646207e0_f64) * t965 * t77863 * t973 + F::cast_from(0.10254018858216406658e4_f64) * t41756 * t23717 + F::cast_from(0.96491876992155210402e2_f64) * t52820 * t6177 - F::cast_from(0.19298375398431042081e3_f64) * t41779 * t23723;
    (t77863, t77873)
}
