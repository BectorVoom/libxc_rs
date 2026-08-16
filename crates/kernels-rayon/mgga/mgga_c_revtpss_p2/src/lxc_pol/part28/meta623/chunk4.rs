//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2211/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2211(t25526: f64, t4820: f64, t15769: f64, t25522: f64, t15687: f64, t25515: f64, t3317: f64, t15693: f64, t16172: f64, t1671: f64, t25512: f64, t4869: f64, t93570: f64, t93573: f64, t93579: f64, t93583: f64, t93585: f64, t93761: f64, t93774: f64) -> (f64, f64) {
    let t100048 = t25526 * t4820;
    let t100051 = 0.3811023832717309953e-3_f64 * t25522 * t15769;
    let t100054 = t25515 * t15687;
    let t100055 = t3317 * t100054;
    let t100058 = 0.57165357490759649296e-3_f64 * t93570 - 0.28582678745379824648e-3_f64 * t93573 + 0.3811023832717309953e-3_f64 * t93579 + 0.19055119163586549765e-3_f64 * t93583 + 0.31758531939310916275e-3_f64 * t93585 - 0.47637797908966374413e-3_f64 * t25522 * t16172 + 0.42874018118069736972e-3_f64 * t93761 * t1671 + 0.14481890564325777821e-1_f64 * t93774 * t1671 - 0.30488190661738479624e-2_f64 * t100048 - t100051 + 0.85748036236139473944e-3_f64 * t25512 * t4869 - 0.57165357490759649296e-3_f64 * t100055 * t15693;
    (t100054, t100058)
}
