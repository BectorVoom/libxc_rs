//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1092/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1092<F: Float>(t25304: F, t7057: F, t25301: F, t11007: F, t233: F, t2470: F, t7059: F, t7064: F, t1949: F, t785: F, t780: F, t2439: F) -> (F, F, F, F, F, F, F, F) {
    let t25305 = t25304 * t7057;
    let t25307 = F::new(0.22849835011101738147e-2) * t25305 * t25301;
    let t25317 = t11007 * t233;
    let t25331 = t7059 * t2470;
    let t25333 = F::new(0.17135234354032049604e-1) * t7064 * t25331;
    let t25334 = t785 * t1949;
    let t25335 = t25334 * t780;
    let t25337 = F::new(0.65049603595885220126e-3) * t2439 * t25335;
    (t25305, t25307, t25317, t25331, t25333, t25334, t25335, t25337)
}
