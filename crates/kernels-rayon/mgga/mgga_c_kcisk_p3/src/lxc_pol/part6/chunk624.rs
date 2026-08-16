//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 624/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk624(t1835: f64, t8518: f64, t706: f64, t8510: f64, t8514: f64, t1842: f64, t158: f64, t165: f64, t1809: f64, t1850: f64, t5089: f64, t6903: f64, t6906: f64, t6922: f64, t6924: f64, t6926: f64, t7715: f64, t7718: f64) -> (f64, f64, f64, f64, f64) {
    let t8640 = t1835 * t8518;
    let t8643 = t706 * t8510;
    let t8649 = t1835 * t8514;
    let t8652 = t1842 * t8514;
    let t8661 = -0.23911438650126355246e-1_f64 * t5089 * t7715 - 0.3513e-2_f64 * t158 * t8640 + 0.1171e-2_f64 * t158 * t8643 + 0.9368e-2_f64 * t6922 - 0.26416666666666666666e-2_f64 * t6924 - 0.23526125e-4_f64 * t6926 + 0.7026e-2_f64 * t158 * t8649 - 0.1585e-2_f64 * t165 * t8652 - 0.23911438650126355246e-1_f64 * t6903 + 0.20718155631185227504e-3_f64 * t6906 + 0.11955719325063177623e-1_f64 * t1809 * t7718 - 0.5179538907796306876e-4_f64 * t1850 * t7718;
    (t8640, t8643, t8649, t8652, t8661)
}
