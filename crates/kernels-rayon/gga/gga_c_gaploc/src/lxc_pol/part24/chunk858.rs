//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 858/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk858(t8117: f64, t8174: f64, t8220: f64, t8256: f64, t8303: f64, t8343: f64, t8392: f64, t8432: f64, t2967: f64, t747: f64, t1052: f64, t1961: f64) -> (f64, f64, f64) {
    let t8435 = t8117 + t8174 + t8220 + t8256 + t8303 + t8343 + t8392 + t8432;
    let t8440 = t2967 * t747;
    let t8443 = t1052 * t1961;
    (t8435, t8440, t8443)
}
