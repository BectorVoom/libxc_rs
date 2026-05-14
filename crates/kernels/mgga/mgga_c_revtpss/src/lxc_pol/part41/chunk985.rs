//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 985/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk985<F: Float>(t2482: F, t814: F, t823: F, t136: F, t853: F, t220: F, t2723: F, t775: F, t820: F, t844: F, t2681: F, t839: F, t222: F, t9727: F, t2737: F, t9802: F) -> (F, F, F, F, F, F, F, F) {
    let t10777 = t2482 * t823 * t814;
    let t10778 = t853 * t136;
    let t10779 = t10778 * t220;
    let t10786 = t2723 * t775;
    let t10811 = t820 * t823 * t844;
    let t10815 = t820 * t823 * t2681;
    let t10816 = t10815 * t839;
    let t10824 = 455.0 / 1296.0 * t9727 * t222;
    let t10826 = 0.45738002528356795401e-4 * t9802 * t2737;
    (t10777, t10779, t10786, t10811, t10815, t10816, t10824, t10826)
}
