//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 237/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk237<F: Float>(t43: F, t50: F, t316: F, t880: F, t243: F, t75: F, t288: F, t98: F, t47: F, t818: F, t824: F, t100: F, t52: F, t830: F, t833: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t882 = F::new(0.65854491829355115987e0) * t316 * t880;
    let t883 = t243 * t75;
    let t884 = t883 * t288;
    let t885 = F::new(0.11696447245269292414e1) * t884;
    let t886 = F::new(1.0) / t98;
    let t892 = piecewise3::<f64>(t44, F::new(0.0), F::new(4.0) / F::new(9.0) * t886 * t818 + F::new(4.0) / F::new(3.0) * t47 * t824);
    let t893 = F::new(1.0) / t100;
    let t899 = piecewise3::<f64>(t51, F::new(0.0), F::new(4.0) / F::new(9.0) * t893 * t830 + F::new(4.0) / F::new(3.0) * t52 * t833);
    (t882, t883, t884, t885, t886, t892, t893, t899)
}
