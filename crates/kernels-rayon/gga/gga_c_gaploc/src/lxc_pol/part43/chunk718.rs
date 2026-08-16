//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 718/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk718(t13861: f64, t1445: f64, t833: f64, t12218: f64, t935: f64, t2087: f64, t12573: f64, t12574: f64, t13087: f64, t13088: f64, t13091: f64, t13092: f64, t13093: f64, t13094: f64, t13095: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13862 = t1445 * t13861;
    let t13863 = t833 * t13862;
    let t13865 = t12218 * t935;
    let t13866 = t1445 * t13865;
    let t13867 = t2087 * t13866;
    let t13870 = t13087 + t13088 / 2.0_f64 + t12573 - t12574 - t13091 - t13092 + t13093 + t13094 + t13095;
    (t13862, t13863, t13865, t13866, t13867, t13870)
}
