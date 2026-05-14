//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1083/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1083<F: Float>(t25610: F, t27668: F, t25698: F, t378: F, t8521: F, t1989: F, t41937: F, t2453: F, t555: F, t25898: F, t25304: F, t25876: F, t25931: F, t25894: F, t25945: F, t9285: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t94085 = t25610 * t27668;
    let t94121 = t25698 * t378;
    let t94122 = t94121 * t8521;
    let t94149 = t1989 * t41937;
    let t94382 = t2453 * t555;
    let t94383 = t94382 * t25898;
    let t94390 = t25304 * t555;
    let t94391 = t94390 * t25898;
    let t94394 = t25876 * t25931;
    let t94395 = t25894 * t94394;
    let t94407 = t25945 * t9285;
    (t94085, t94122, t94149, t94382, t94383, t94390, t94391, t94394, t94395, t94407)
}
