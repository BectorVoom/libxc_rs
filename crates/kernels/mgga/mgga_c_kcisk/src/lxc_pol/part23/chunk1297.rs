//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1297/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1297<F: Float>(t140: F, t15224: F, t190: F, t15189: F, t110934: F, t9379: F, t110940: F, t9365: F, t110996: F, t110999: F, t111001: F, t111003: F, t111007: F, t111010: F, t111013: F, t32652: F, t32658: F) -> (F, F, F, F) {
    let t111016 = t140 * t15224 * t190;
    let t111019 = t140 * t15189 * t190;
    let t111021 = t9379 * t110934;
    let t111023 = t9365 * t110940;
    let t111025 = 0.31250000000000000001e-1 * t110996 + 0.24125000000000000001e-1 * t110999 + 0.120625e-1 * t111001 - 0.72916666666666666668e-1 * t111003 + 0.40208333333333333335e-2 * t111007 - 0.14583333333333333334e0 * t111010 - 0.27857666666666666666e-1 * t111013 - 0.72223580246913580243e-1 * t111016 + 0.69644166666666666665e-2 * t111019 + 0.62500000000000000002e-1 * t111021 - 0.36187500000000000001e-1 * t111023;
    let t111026 = t32652 * t32658;
    (t111016, t111019, t111025, t111026)
}
