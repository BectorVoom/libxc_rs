//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1215/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1215<F: Float>(t820: F, t823: F, t844: F, t2681: F, t839: F, t222: F, t9727: F, t2737: F, t9802: F, t2482: F, t596: F, t2487: F) -> (F, F, F, F, F, F, F) {
    let t10811 = t820 * t823 * t844;
    let t10815 = t820 * t823 * t2681;
    let t10816 = t10815 * t839;
    let t10824 = F::new(455.0) / F::new(1296.0) * t9727 * t222;
    let t10826 = F::cast_from(0.45738002528356795401e-4_f64) * t9802 * t2737;
    let t10845 = t2482 * t823 * t596;
    let t10846 = t10845 * t2487;
    (t10811, t10815, t10816, t10824, t10826, t10845, t10846)
}
