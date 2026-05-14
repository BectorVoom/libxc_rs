//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1159/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1159<F: Float>(t1544: F, t6075: F, t25207: F, t1583: F, t27383: F, t1468: F, t29598: F, t98658: F, t198: F, t23114: F, t105934: F, t105937: F, t105939: F, t105947: F, t105949: F, t105954: F, t105956: F, t105960: F, t105962: F, t106353: F, t1580: F, t92861: F, t92870: F, t92873: F, t98825: F) -> (F, F, F, F, F, F, F, F) {
    let t113103 = t1544 * t6075;
    let t113104 = t25207 * t113103;
    let t113107 = t1583 * t6075;
    let t113108 = t27383 * t113107;
    let t113111 = t1468 * t6075;
    let t113115 = t98658 * t29598;
    let t113123 = t198 * t23114;
    let t113138 = 0.43368140941025997312e-1 * t105934 - 0.15421710918628844643e0 * t105937 - 0.77108554593144223218e-1 * t105939 + 0.21684070470512998656e-1 * t105947 + 0.86736281882051994623e-1 * t105949 + 0.21684070470512998656e-1 * t105954 - 0.38554277296572111609e-1 * t105956 + 0.51405703062096148812e-1 * t98825 - 0.43368140941025997312e-1 * t105960 + 0.77108554593144223218e-1 * t105962 + t92861 - 0.19756347548806534796e1 * t106353 * t1580 - t92870 - t92873;
    (t113103, t113104, t113107, t113108, t113111, t113115, t113123, t113138)
}
