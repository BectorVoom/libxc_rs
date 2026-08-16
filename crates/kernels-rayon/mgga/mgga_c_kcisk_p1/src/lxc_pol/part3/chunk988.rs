//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 988/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk988(t13944: f64, t6332: f64, t6331: f64, t1483: f64, t4313: f64, t1512: f64, t4305: f64, t493: f64, t1517: f64, t4301: f64, t1493: f64, t4297: f64) -> (f64, f64, f64, f64, f64) {
    let t14555 = t6332 * t13944;
    let t14556 = t6331 * t14555;
    let t14558 = t1483 * t4313;
    let t14560 = t1512 * t4305;
    let t14561 = t493 * t14560;
    let t14563 = t4301 * t1517;
    let t14565 = t4297 * t1493;
    (t14556, t14558, t14561, t14563, t14565)
}
