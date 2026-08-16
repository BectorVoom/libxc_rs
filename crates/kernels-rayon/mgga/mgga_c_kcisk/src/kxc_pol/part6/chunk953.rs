//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 953/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk953(t772: f64, t29754: f64, t2021: f64, t1586: f64, t2642: f64, t9176: f64, t12255: f64, t28368: f64, t5486: f64, t1775: f64, t2629: f64, t9183: f64, t2633: f64, t9207: f64) -> (f64, f64, f64, f64, f64) {
    let t783 = 0.0_f64 < t772;
    let t29845 = piecewise3(t783, t29754, -t29754);
    let t29846 = t2021 * t29845;
    let t29847 = t1586 * t29846;
    let t29852 = t9176 * t2642;
    let t29853 = t12255 * t29852;
    let t29854 = t1586 * t29853;
    let t29861 = t5486 * t28368;
    let t29862 = t1775 * t29861;
    let t29867 = t2629 * t9183;
    let t29870 = t9207 * t2633;
    (t29847, t29854, t29862, t29867, t29870)
}
