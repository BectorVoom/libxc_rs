//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1203/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1203<F: Float>(t126089: F, t119790: F, t121806: F, t121810: F, t121815: F, t126081: F, t126083: F, t126085: F, t126087: F, t126095: F, t1955: F, t1959: F, t28340: F) -> F {
    let t127620 = F::new(0.13223814266738539448e-3) * t126089;
    let t127628 = -F::new(0.29749863367240808656e-2) * t126081 + F::new(0.7437465841810202164e-3) * t126083 + F::new(0.7437465841810202164e-3) * t126085 - F::new(0.74374658418102021639e-4) * t126087 + t127620 - F::new(0.25702851531048074406e-1) * t121806 - F::new(0.8673628188205199462e0) * t1955 * t28340 * t1959 + F::new(0.28559868832551176308e-1) * t121810 + t119790 + F::new(0.14456046980341999104e-1) * t121815 + F::new(0.56468933516960933999e-3) * t126095;
    t127628
}
