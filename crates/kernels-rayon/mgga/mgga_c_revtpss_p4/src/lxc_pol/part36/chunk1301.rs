//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1301/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1301(t29598: f64, t98658: f64, t198: f64, t23114: f64, t105934: f64, t105937: f64, t105939: f64, t105947: f64, t105949: f64, t105954: f64, t105956: f64, t105960: f64, t105962: f64, t106353: f64, t1580: f64, t92861: f64, t92870: f64, t92873: f64, t98825: f64) -> (f64, f64, f64) {
    let t113115 = t98658 * t29598;
    let t113123 = t198 * t23114;
    let t113138 = 0.43368140941025997312e-1_f64 * t105934 - 0.15421710918628844643e0_f64 * t105937 - 0.77108554593144223218e-1_f64 * t105939 + 0.21684070470512998656e-1_f64 * t105947 + 0.86736281882051994623e-1_f64 * t105949 + 0.21684070470512998656e-1_f64 * t105954 - 0.38554277296572111609e-1_f64 * t105956 + 0.51405703062096148812e-1_f64 * t98825 - 0.43368140941025997312e-1_f64 * t105960 + 0.77108554593144223218e-1_f64 * t105962 + t92861 - 0.19756347548806534796e1_f64 * t106353 * t1580 - t92870 - t92873;
    (t113115, t113123, t113138)
}
