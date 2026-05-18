//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1141/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1141<F: Float>(t20693: F, t19756: F, t19760: F, t19764: F, t19768: F, t19772: F, t19774: F, t19776: F, t19780: F, t19784: F, t19788: F, t19793: F, t19796: F, t19799: F, t19804: F, t19807: F, t19811: F, t19814: F, t19816: F, t19818: F, t9215: F) -> (F, F) {
    let t20694 = t20693 / F::new(45.0);
    let t20715 = -F::new(0.04534) * t19756 + F::new(0.003778333333333333) * t19760 + F::new(0.007556666666666666) * t19764 - F::new(0.04534) * t19768 + F::new(0.06801) * t19772 - F::new(0.0012594444444444445) * t19774 + F::new(0.003778333333333333) * t19776 - F::new(0.011335) * t19780 - F::new(0.02267) * t19784 + F::new(0.04534) * t19788 - F::new(0.02518888888888889) * t19793 + F::new(0.005597530864197531) * t19796 + F::new(0.012594444444444445) * t19799 - F::new(0.003778333333333333) * t19804 + F::new(0.0012594444444444445) * t19807 - F::new(0.0019591358024691357) * t9215 - F::new(0.02267) * t19811 + F::new(0.006297222222222222) * t19814 - F::new(0.0006996913580246914) * t19816 - F::new(0.0006297222222222223) * t19818;
    (t20694, t20715)
}
