//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1204/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1204<F: Float>(t121818: F, t121820: F, t121825: F, t121827: F, t121830: F, t121836: F, t121838: F, t121840: F, t126099: F, t32430: F, t32441: F, t34075: F) -> F {
    let t127640 = -F::new(0.56468933516960933999e-3) * t126099 + F::new(0.42839803248826764462e-1) * t121818 - F::new(0.25702851531048074406e-1) * t121820 - t121825 - F::new(0.14279934416275588154e-1) * t121827 + F::new(0.57119737665102352616e0) * t34075 * t32430 + F::new(0.57119737665102352616e0) * t34075 * t32441 + F::new(0.14456046980341999104e-1) * t121830 - t121836 + F::new(0.25389723392137995738e-1) * t121838 - t121840;
    t127640
}
