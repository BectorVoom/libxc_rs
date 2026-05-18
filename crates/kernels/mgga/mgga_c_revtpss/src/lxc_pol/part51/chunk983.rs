//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 983/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk983<F: Float>(t5: F, t33620: F, t8621: F, t1493: F, t84: F, t32136: F, t32142: F, t32149: F, t32154: F, t33609: F, t33613: F, t33617: F, t8443: F) -> (F, F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t33621 = t8621 * t33620;
    let t33624 = t84 * t1493;
    let t33625 = t8621 * t33624;
    let t33629 = piecewise3::<f64>(t8, F::new(0.0), F::new(5.0) / F::new(144.0) * t33609 * t8443 - F::new(5.0) / F::new(24.0) * t32136 * t33613 - F::new(5.0) / F::new(36.0) * t32142 * t33617 + F::new(5.0) / F::new(72.0) * t32149 * t33621 + F::new(5.0) / F::new(72.0) * t32154 * t33625);
    (t33621, t33624, t33625, t33629)
}
