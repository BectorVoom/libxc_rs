//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1402/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1402(t34826: f64, t10421: f64, t20887: f64, t10424: f64, t30733: f64, t10569: f64, t10570: f64, t10573: f64, t10591: f64, t1580: f64, t1628: f64, t3358: f64, t34790: f64, t34794: f64, t34797: f64, t34801: f64, t34817: f64, t34821: f64, t34823: f64, t4585: f64, t541: f64, t557: f64, t597: f64) -> f64 {
    let t34827 = 0.89376224879626066674e-1_f64 * t34826;
    let t34828 = t10421 * t20887;
    let t34829 = 0.14896037479937677779e-1_f64 * t34828;
    let t34830 = t10424 * t30733;
    let t34831 = 0.59584149919750711116e-1_f64 * t34830;
    let t34832 = t34790 - t34794 - t34797 - t34801 + 0.46011511144704899612e1_f64 * t1580 * t10570 + 0.47667319935800568892e0_f64 * t10591 * t541 + 0.79445533226334281487e-1_f64 * t557 * t4585 * t3358 + 0.61348681526273199482e1_f64 * t1580 * t10573 + 0.61348681526273199482e1_f64 * t597 * t1628 * t10569 + t34817 - t34821 + t34823 - t34827 - t34829 - t34831;
    t34832
}
