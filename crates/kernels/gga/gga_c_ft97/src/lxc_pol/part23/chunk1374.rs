//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1374/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1374<F: Float>(t19100: F, t19116: F, t1208: F, t668: F, t122796: F, t7006: F, t108448: F, t31409: F, t684: F, t111830: F, t111838: F, t112156: F, t123421: F, t127167: F, t127395: F, t127492: F, t127530: F, t127534: F, t127564: F, t19101: F, t19132: F, t19135: F, t25077: F, t27642: F, t28547: F, t28558: F, t28562: F, t28566: F, t31502: F, t4104: F, t70457: F, t98520: F, t98544: F, t98545: F, t98581: F) -> (F,) {
    let t127614 = t19116 * t19100;
    let t127626 = t1208 * t668;
    let t127633 = t7006 * t122796;
    let t127636 = t108448 * t31409 * t684;
    let t127647 = 0.18611243628760286395e2 * t19132 * t127530 - 0.93056218143801431977e1 * t19135 * t127534 + 0.43791161479435967988e1 * t127614 * t31502 + 0.43791161479435967988e1 * t19101 * t127564 - 0.45306850413028723348e0 * t4104 * t127492 - 0.17780800291358024692e0 * t25077 * t27642 * t28562 - t98581 - 0.10741227453659940874e0 * t28558 * t123421 - 0.20003400327777777778e0 * t98520 * t98545 * t127626 * t28547 - 0.18122740165211489339e1 * t70457 * t127167 - 0.46992870109762241323e0 * t127633 + 0.1611184118048991131e0 * t111830 * t127636 - 0.1611184118048991131e0 * t112156 * t127636 - 0.1611184118048991131e0 * t111838 * t127395 + 0.13335600218518518519e0 * t98544 * t98545 * t127626 * t28566;
    (t127647,)
}
