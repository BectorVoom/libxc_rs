//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 522/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk522<F: Float>(t10646: F, t10631: F, t10634: F, t10638: F, t10642: F, t10645: F, t9618: F, t9620: F, t9622: F, t9627: F, t9629: F, t9632: F, t9635: F, t9651: F, t9664: F, t9666: F, t9669: F, t9672: F, t9674: F, t9676: F) -> (F, F) {
    let t10647 = 0.42725145723012357132e-3 * t10646;
    let t10648 = t9618 - t9620 - t9622 - t9627 + t9629 + t9632 - t10631 + t10634 - t10638 - t10642 - t10645 - t10647 - t9635 - t9651;
    let t10657 = -21.0 / 256.0 * t9664 + 147.0 / 8192.0 * t9666 - 63.0 / 524288.0 * t9669 + 21.0 / 524288.0 * t9672 - 49.0 / 8192.0 * t9674 + 7.0 / 256.0 * t9676;
    (t10648, t10657)
}
