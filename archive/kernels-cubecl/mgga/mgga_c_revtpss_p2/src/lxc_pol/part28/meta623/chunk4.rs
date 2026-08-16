//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2211/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2211<F: Float>(t25526: F, t4820: F, t15769: F, t25522: F, t15687: F, t25515: F, t3317: F, t15693: F, t16172: F, t1671: F, t25512: F, t4869: F, t93570: F, t93573: F, t93579: F, t93583: F, t93585: F, t93761: F, t93774: F) -> (F, F) {
    let t100048 = t25526 * t4820;
    let t100051 = F::cast_from(0.3811023832717309953e-3_f64) * t25522 * t15769;
    let t100054 = t25515 * t15687;
    let t100055 = t3317 * t100054;
    let t100058 = F::cast_from(0.57165357490759649296e-3_f64) * t93570 - F::cast_from(0.28582678745379824648e-3_f64) * t93573 + F::cast_from(0.3811023832717309953e-3_f64) * t93579 + F::cast_from(0.19055119163586549765e-3_f64) * t93583 + F::cast_from(0.31758531939310916275e-3_f64) * t93585 - F::cast_from(0.47637797908966374413e-3_f64) * t25522 * t16172 + F::cast_from(0.42874018118069736972e-3_f64) * t93761 * t1671 + F::cast_from(0.14481890564325777821e-1_f64) * t93774 * t1671 - F::cast_from(0.30488190661738479624e-2_f64) * t100048 - t100051 + F::cast_from(0.85748036236139473944e-3_f64) * t25512 * t4869 - F::cast_from(0.57165357490759649296e-3_f64) * t100055 * t15693;
    (t100054, t100058)
}
