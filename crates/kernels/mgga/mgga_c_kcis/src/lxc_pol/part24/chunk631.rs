//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 631/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk631<F: Float>(t1220: F, t6804: F, t3577: F, t6788: F, t6406: F, t971: F, t6423: F, t3034: F, t1212: F, t1225: F, t1831: F, t1835: F, t3550: F, t3575: F, t3585: F, t3592: F, t405: F, t5211: F, t5242: F, t6362: F, t6364: F, t6368: F, t6392: F, t6395: F, t6401: F, t6783: F, t6789: F) -> (F, F, F, F, F, F) {
    let t6805 = t6804 * t1220;
    let t6808 = t6788 * t3577;
    let t6814 = t6406 * t971;
    let t6817 = t6423 * t971;
    let t6820 = t6406 * t3034;
    let t6823 = -F::new(0.3109e-1) * t6783 * t405 + F::new(2.0) * t5211 * t1831 - F::new(2.0) * t3550 * t6789 + F::new(1.0) * t1212 * t6805 + F::new(0.32164683177870697974e2) * t3575 * t6808 + t6362 - t6364 + t6368 - t6392 - t6395 - F::new(0.19751789702565206229e-1) * t6401 + F::new(0.11696446794910408142e1) * t5242 * t1835 - F::new(0.11696446794910408142e1) * t3585 * t6814 + F::new(0.58482233974552040708e0) * t1225 * t6817 + F::new(0.17315755899375863299e2) * t3592 * t6820;
    (t6805, t6808, t6814, t6817, t6820, t6823)
}
