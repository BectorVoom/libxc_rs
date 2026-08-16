//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 716/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk716(t2471: f64, t558: f64, t1743: f64, t699: f64, t8125: f64, t8710: f64, t8714: f64, t8716: f64, t8718: f64, t8735: f64, t9874: f64, t9878: f64, t9880: f64, t9882: f64, t9886: f64, t9890: f64, t9892: f64, t9894: f64, t9897: f64, t9899: f64) -> (f64, f64, f64) {
    let t10417 = t2471 * t558;
    let t10420 = t699 * t1743;
    let t10443 = 0.68186654135613354324e-2_f64 * t9874 - 0.90915538847484472432e-2_f64 * t9878 + 0.1814407727691612783e-3_f64 * t9880 - 0.21168090156402149135e-3_f64 * t9882 + 0.13637330827122670865e-1_f64 * t9886 + 0.45457769423742236216e-1_f64 * t9890 + 0.9072038638458063915e-3_f64 * t9892 + 0.16934472125121719308e-2_f64 * t9894 - 0.9676841214355268176e-3_f64 * t8710 + 0.11289648083414479539e-2_f64 * t8714 - 0.36366215538993788972e-1_f64 * t8716 + 0.48488287385325051964e-1_f64 * t8718 + 0.79656924630363488034e-2_f64 * t9897 + 0.11974241701863808564e0_f64 * t9899 + t8125 + 0.35403077613494883571e-2_f64 * t8735;
    (t10417, t10420, t10443)
}
