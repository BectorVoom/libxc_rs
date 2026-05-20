//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 894/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk894<F: Float>(t1376: F, t2689: F, t1353: F, t1413: F, t547: F, t807: F, t2700: F, t535: F, t1369: F, t794: F) -> (F, F, F, F, F, F) {
    let t3950 = F::cast_from(0.76220476654346199061e-4_f64) * t2689 * t1376;
    let t3951 = t1413 * t1353;
    let t3952 = t547 * t3951;
    let t3953 = t807 * t3952;
    let t3956 = F::new(35.0) / F::new(432.0) * t2700 * t535;
    let t3957 = t794 * t1369;
    (t3950, t3951, t3952, t3953, t3956, t3957)
}
