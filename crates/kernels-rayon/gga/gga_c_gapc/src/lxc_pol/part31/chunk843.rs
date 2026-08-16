//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 843/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk843(t1092: f64, t2555: f64, t191: f64, t2786: f64, t3304: f64, t9556: f64, t9558: f64, t9561: f64, t9565: f64, t9568: f64, t9570: f64, t9572: f64, t9579: f64, t9581: f64, t9584: f64, t9587: f64) -> (f64, f64, f64) {
    let t9589 = t1092 * t2555;
    let t9591 = t2786 * t191;
    let t9592 = t9591 * t3304;
    let t9594 = -0.49196596498842592595e-6_f64 * t9556 - 0.16908181191593721013e-4_f64 * t9558 + 0.72463633678258804342e-6_f64 * t9561 + 0.84410248952307505288e-7_f64 * t9565 + 0.16882049790461501058e-6_f64 * t9568 - 0.30660168560756614104e-3_f64 * t9570 + 0.4637672555408563478e-4_f64 * t9572 - 0.84410248952307505288e-7_f64 * t9579 - 0.98393192997685185188e-5_f64 * t9581 + 0.38010404803226280926e-3_f64 * t9584 + 0.14492726735651760868e-5_f64 * t9587 + 0.33816362383187442026e-4_f64 * t9589 - 0.14492726735651760868e-5_f64 * t9592;
    (t9589, t9592, t9594)
}
