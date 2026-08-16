//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 892/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk892(t11443: f64, t28414: f64, t706: f64, t11417: f64, t11418: f64, t28368: f64, t2488: f64, t8536: f64, t7055: f64, t1876: f64, t4598: f64, t11328: f64, t4595: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28885 = t11443 * t28414;
    let t28886 = t706 * t28885;
    let t28894 = t11417 * t11418 * t28368;
    let t28897 = t2488 * t8536;
    let t28898 = t7055 * t28897;
    let t28902 = t1876 * t4598 * t28368;
    let t28906 = t4595 * t11328 * t28368;
    (t28885, t28886, t28894, t28897, t28898, t28902, t28906)
}
