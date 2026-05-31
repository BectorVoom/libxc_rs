//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 589/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk589<F: Float>(t1963: F, t30: F, t1940: F, t343: F, t43: F, t136: F, t359: F, t365: F, sigma0: F) -> (F, F, F, F, F) {
    let t1964 = t1963 * t30;
    let t1966 = t1940 * t1964 / F::cast_from(2.0_f64);
    let t1967 = t43 * t343;
    let t1968 = t1967 * t136;
    let t1971 = t359 * sigma0;
    let t1972 = t1971 * t365;
    (t1966, t1967, t1968, t1971, t1972)
}
