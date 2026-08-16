//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1000/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1000(t2723: f64, t775: f64, t820: f64, t823: f64, t844: f64, t2681: f64, t839: f64, t222: f64, t9727: f64, t2737: f64, t9802: f64, t2482: f64, t596: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10786 = t2723 * t775;
    let t10811 = t820 * t823 * t844;
    let t10815 = t820 * t823 * t2681;
    let t10816 = t10815 * t839;
    let t10824 = 455.0_f64 / 1296.0_f64 * t9727 * t222;
    let t10826 = 0.45738002528356795401e-4_f64 * t9802 * t2737;
    let t10845 = t2482 * t823 * t596;
    (t10786, t10811, t10815, t10816, t10824, t10826, t10845)
}
