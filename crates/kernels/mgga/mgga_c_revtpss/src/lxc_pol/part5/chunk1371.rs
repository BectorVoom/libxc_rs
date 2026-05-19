//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1371/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1371<F: Float>(t5: F, t21812: F, t117: F, t5892: F, t625: F, t10208: F, t5891: F, t665: F, t4263: F, t4287: F, t5916: F, t2339: F, t5915: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t21813 = piecewise3::<F>(t8, F::new(0.0), t21812);
    let t21814 = t21813 * t117;
    let t21818 = t625 * t5892;
    let t21820 = t10208 * t5891;
    let t21821 = t21820 * t665;
    let t21824 = t4263 * t4287;
    let t21827 = t625 * t5916;
    let t21829 = t2339 * t5915;
    (t21814, t21818, t21821, t21824, t21827, t21829)
}
