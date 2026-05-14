//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 778/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk778<F: Float>(t3869: F, t9575: F, t1331: F, t3860: F, t1320: F, t3855: F, t186: F, t685: F, t793: F, t1337: F, t4135: F, t5541: F, t7315: F, t9514: F, t9517: F, t9521: F, t9560: F, t9562: F, t9565: F, t9567: F, t9569: F, t9571: F, t9574: F) -> (F, F, F, F, F, F) {
    let t9577 = 0.21687162600603479684e-1 * t3869 * t9575;
    let t9578 = t3860 * t1331;
    let t9579 = 36.0 * t9578;
    let t9580 = t1320 * t3855;
    let t9581 = 12.0 * t9580;
    let t9586 = t685 * t793 * t186;
    let t9588 = 0.56968947174242584612e-3 * t1337 * t9586;
    let t9589 = -3.0 * t4135 * t5541 * t7315 + t9514 - t9517 - t9521 + t9560 + t9562 - t9565 + t9567 + t9569 - t9571 - t9574 - t9577 + t9579 - t9581 - t9588;
    (t9577, t9579, t9581, t9586, t9588, t9589)
}
