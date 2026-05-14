//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 704/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk704<F: Float>(t609: F, t1307: F, t1610: F, t4440: F, t1444: F, t617: F, t2642: F, t1600: F, t1601: F, t2645: F, t1606: F, t616: F, t494: F, t286: F, t4390: F, t1608: F, t1599: F, t4424: F, t4427: F, t4430: F, t4435: F, t4439: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t614 = 0.0 < t609;
    let t4441 = t1307 * t1610;
    let t4442 = t4440 * t4441;
    let t4445 = t617 * t1444;
    let t4446 = t4445 * t2642;
    let t4447 = t1600 * t4446;
    let t4450 = t1601 * t2645;
    let t4451 = t1600 * t4450;
    let t4455 = 1.0 / t1606 / t616;
    let t4456 = t494 * t4455;
    let t4457 = t1610 * t1610;
    let t4458 = t4456 * t4457;
    let t4459 = t286 * t4458;
    let t4463 = piecewise3(t614, t4390, -t4390);
    let t4464 = t1608 * t4463;
    let t4465 = t286 * t4464;
    let t4468 = -t4424 + t4427 / 864.0 - t4430 / 288.0 + t1599 * t4435 / 432.0 - t4439 * t4442 / 288.0 - t1599 * t4447 / 288.0 + t1599 * t4451 / 576.0 + t1599 * t4459 / 96.0 - t1599 * t4465 / 192.0;
    (t4441, t4442, t4446, t4447, t4450, t4451, t4455, t4457, t4458, t4459, t4463, t4464, t4465, t4468)
}
