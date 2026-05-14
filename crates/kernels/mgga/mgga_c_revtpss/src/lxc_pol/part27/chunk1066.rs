//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1066/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1066<F: Float>(t2439: F, t25334: F, t887: F, t2722: F, t7048: F, t10799: F, t27261: F, t10773: F, t25270: F, t10766: F, t10794: F, t7036: F, t820: F, t844: F, t2751: F, t2482: F, t814: F) -> (F, F, F, F, F, F, F, F) {
    let t92935 = t2439 * t25334 * t887;
    let t92937 = t7048 * t2722;
    let t92942 = t27261 * t10799;
    let t92944 = t25270 * t10773;
    let t92946 = t25270 * t10766;
    let t92948 = t25270 * t10794;
    let t92951 = t820 * t7036 * t844;
    let t92952 = t92951 * t2751;
    let t92955 = t2482 * t7036 * t814;
    (t92935, t92937, t92942, t92944, t92946, t92948, t92952, t92955)
}
